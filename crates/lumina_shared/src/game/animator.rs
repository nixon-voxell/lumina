use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;

// For docs.
#[allow(unused_imports)]
use bevy::animation::ActiveAnimation;

pub(super) struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                advance_animator,
                track_delay_timer,
                update_animation_physics,
            ),
        )
        .add_systems(Update, (init_animation_player, update_animation_player));
    }
}

fn init_animation_player(
    q_animators: Query<
        (
            &Animator,
            &BlueprintAnimationPlayerLink,
            &BlueprintAnimations,
        ),
        Added<BlueprintAnimations>,
    >,
    mut q_players: Query<&mut AnimationPlayer>,
) {
    for (animator, link, animations) in q_animators.iter() {
        let Ok(mut player) = q_players.get_mut(link.0) else {
            continue;
        };

        for &index in animator
            .names
            .iter()
            .filter_map(|name| animations.named_indices.get(name))
        {
            player.play(index).set_speed(0.0);
        }
    }
}

fn update_animation_player(
    q_animators: Query<(
        &Animator,
        &BlueprintAnimationPlayerLink,
        &BlueprintAnimations,
    )>,
    mut q_players: Query<&mut AnimationPlayer>,
    fixed_time: Res<Time<Fixed>>,
) {
    let overstep_frac = fixed_time.overstep_fraction();
    for (animator, link, animations) in q_animators.iter() {
        let Ok(mut player) = q_players.get_mut(link.0) else {
            continue;
        };

        for &index in animator
            .names
            .iter()
            .filter_map(|name| animations.named_indices.get(name))
        {
            player
                .play(index)
                .seek_to(animator.prev_time.lerp(animator.time, overstep_frac));
        }
    }
}

fn update_animation_physics(
    mut q_players: Query<
        (
            &Position,
            &mut Rotation,
            &mut LinearVelocity,
            &GlobalTransform,
        ),
        With<AnimationPlayer>,
    >,
    time: Res<Time<Fixed>>,
) {
    const VELOCITY_DAMP: f32 = 0.8;
    let delta_seconds = time.delta_seconds();

    for (position, mut rotation, mut linear_velocity, global_transform) in q_players.iter_mut() {
        let transform = global_transform.compute_transform();

        // Damp the velocities.
        linear_velocity.0 =
            ((transform.translation.xy() - position.0) / delta_seconds) * VELOCITY_DAMP;
        // Directly manipulate rotations.
        *rotation = Rotation::radians(transform.rotation.to_scaled_axis().z);
    }
}

/// Advance [`Playback::time`] based on the [`Playback::time_scale`]
/// and update the [`Animator::time`].
fn advance_animator(
    mut commands: Commands,
    mut q_animators: Query<(&mut Animator, &mut Playback, Entity), Without<DelayTimer>>,
    time: Res<Time>,
) {
    for (mut animator, mut playback, entity) in q_animators.iter_mut() {
        animator.prev_time = animator.time;
        animator.time = playback.time;
        playback.time += time.delta_seconds() * playback.time_scale;

        if playback.time > animator.duration {
            // Playback ended with positive time scale.
            if let RepeatMode::Repeat { delay } | RepeatMode::PingPong { delay } =
                animator.repeat_mode
            {
                commands.entity(entity).insert(DelayTimer(
                    Timer::from_seconds(delay, TimerMode::Once),
                    true,
                ));
                playback.time_scale = 0.0;
            }
        } else if playback.time < -f32::EPSILON {
            // Playback ended with negative time scale.
            if let RepeatMode::PingPong { delay } = animator.repeat_mode {
                commands.entity(entity).insert(DelayTimer(
                    Timer::from_seconds(delay, TimerMode::Once),
                    false,
                ));
                playback.time_scale = 0.0;
            }
        }

        playback.time = playback.time.clamp(0.0, animator.duration);
    }
}

fn track_delay_timer(
    mut commands: Commands,
    mut q_timers: Query<(&mut DelayTimer, &mut Playback, &Animator, Entity)>,
    time: Res<Time>,
) {
    for (mut timer, mut playback, animator, entity) in q_timers.iter_mut() {
        if timer.finished() {
            match animator.repeat_mode {
                RepeatMode::Repeat { .. } => {
                    playback.time_scale = animator.time_scale;
                    playback.time = 0.0;
                }
                RepeatMode::PingPong { .. } => {
                    // Reverse
                    playback.time_scale = match timer.1 {
                        true => -animator.time_scale,
                        false => animator.time_scale,
                    };
                }
                RepeatMode::Once => {}
            }

            commands.entity(entity).remove::<DelayTimer>();
        }

        timer.tick(time.delta());
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Animator {
    /// The total duration of the animation.
    duration: f32,
    /// The names of the animation clips (or "Action" in Blender).
    names: Vec<String>,
    /// Determines how the animations are going to repeat.
    repeat_mode: RepeatMode,
    /// A non-negative time scale for reference when repetition occurs.
    time_scale: f32,
    #[reflect(ignore)]
    /// The current playback time used for seeking the [`ActiveAnimation`].
    time: f32,
    #[reflect(ignore)]
    /// The previous playback time used for seeking the [`ActiveAnimation`].
    /// This is for interpolation between [`FixedUpdate`].
    prev_time: f32,
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct Playback {
    /// Player time scale.
    /// 0.0 to stop playing, positive for forward, and negative for reverse.
    pub time_scale: f32,
    /// The current playback time, tweak this to seek the animator.
    pub time: f32,
}

#[derive(Reflect, Serialize, Deserialize, PartialEq)]
pub enum RepeatMode {
    /// Do not repeat.
    Once,
    /// Repeat normally, play to the end
    /// and then rewind to the start.
    Repeat {
        /// Delay time before the repeat happens.
        delay: f32,
    },
    /// Repeat by playing to the end and then
    /// reversing back to the start.
    PingPong {
        /// Delay time before the repeat happens.
        delay: f32,
    },
}

#[derive(Component, Deref, DerefMut, Serialize, Deserialize, PartialEq)]
pub struct DelayTimer(#[deref] Timer, bool);
