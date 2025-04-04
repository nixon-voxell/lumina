use std::sync::{LazyLock, RwLock};
use std::time::Duration;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use bevy_motiongfx::prelude::ease;
use foundations::{dict, func, Dict};
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimateDuration(0.3))
            .add_systems(PreStartup, define_interactions_func)
            .add_systems(
                Update,
                (
                    disable_specific_interactions,
                    update_animate_direction,
                    update_animate_time,
                    update_interactions,
                    track_animate_time,
                )
                    .chain(),
            );
    }
}

pub static INTERACTIONS: LazyLock<RwLock<HashMap<TypLabel, f64>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[func]
fn interactions() -> Vec<Dict> {
    INTERACTIONS
        .read()
        .unwrap()
        .iter()
        .map(|(&label, &time)| {
            dict! {
                "label" => label,
                "time" => time
            }
        })
        .collect()
}

/// Define the [`interactions`] function into the Typst world.
fn define_interactions_func(world: Res<TypstWorldRef>) {
    let mut world = world.write().unwrap();
    let scope = world.library.global.scope_mut();
    scope.define_func::<interactions>();
}

/// Disable interactions with non-button labels.
fn disable_specific_interactions(
    mut commands: Commands,
    mut q_interactions: Query<(&TypstLabel, Entity), Added<Interaction>>,
) {
    for (label, entity) in q_interactions.iter_mut() {
        if label.resolve().starts_with("btn") == false {
            commands
                .entity(entity)
                .insert(FocusPolicy::Pass)
                .remove::<Interaction>();
        }
    }
}

/// Update [`AnimateDirection`] and initiate [`AnimateTime`] when needed.
fn update_animate_direction(
    mut commands: Commands,
    mut q_interactions: Query<
        (&Interaction, Option<&mut AnimateTime>, Entity),
        (Changed<Interaction>, With<TypstLabel>),
    >,
) {
    for (interaction, animate, entity) in q_interactions.iter_mut() {
        match animate {
            Some(mut animate) => {
                // Change the animation direction.
                if *interaction == Interaction::Hovered {
                    animate.target_direction = AnimateDirection::Forward;
                } else {
                    animate.target_direction = AnimateDirection::Backward;
                }
            }
            None => {
                // Non-hovered interaction at this point is pointless
                // as there isn't any animation time to begin going backwards with.
                if *interaction == Interaction::Hovered {
                    // Start the animation.
                    commands.entity(entity).insert(AnimateTime::default());
                }
            }
        }
    }
}

/// Update [`AnimateTime::time`].
fn update_animate_time(
    mut q_animates: Query<&mut AnimateTime>,
    time: Res<Time>,
    duration: Res<AnimateDuration>,
) {
    for mut animate in q_animates.iter_mut() {
        animate.tick(time.delta(), &duration);
    }
}

/// Remove [`TypLabel`] from [`INTERACTIONS`] when [`AnimateTime`] is removed.
fn update_interactions(
    q_labels: Query<(&TypstLabel, &AnimateTime)>,
    duration: Res<AnimateDuration>,
) {
    let mut interactions = INTERACTIONS.write().unwrap();
    for (label, animate) in q_labels.iter() {
        let animate_time = ease::cubic::ease_in_out(animate.time / duration.0) as f64;

        match interactions.get_mut(&**label) {
            Some(time) => *time = animate_time,
            None => {
                interactions.insert(**label, animate_time);
            }
        }
    }
}

/// Remove [`TypLabel`] from [`INTERACTIONS`] when
/// [`AnimateTime`] is [finished][AnimateTime::is_finished].
fn track_animate_time(
    mut commands: Commands,
    q_animates: Query<(&AnimateTime, &TypstLabel, Entity)>,
) {
    let mut interactions = INTERACTIONS.write().unwrap();
    for (animate, label, entity) in q_animates.iter() {
        if animate.is_finished() {
            commands.entity(entity).remove::<AnimateTime>();
            interactions.remove(&**label);
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct AnimateDuration(f32);

#[derive(Component, Default)]
struct AnimateTime {
    time: f32,
    target_direction: AnimateDirection,
    computed_direction: AnimateDirection,
}

impl AnimateTime {
    fn tick(&mut self, delta: Duration, duration: &AnimateDuration) {
        self.computed_direction = match self.target_direction {
            AnimateDirection::Backward
                if self.time >= duration.0
                    || matches!(self.computed_direction, AnimateDirection::Backward) =>
            {
                AnimateDirection::Backward
            }
            _ => AnimateDirection::Forward,
        };

        let delta_secs = match self.computed_direction {
            AnimateDirection::Forward => delta.as_secs_f32(),
            AnimateDirection::Backward => -delta.as_secs_f32(),
        };

        self.time += delta_secs;
        self.time = self.time.clamp(0.0, duration.0);
    }

    fn is_finished(&self) -> bool {
        self.time <= 0.0
    }
}

#[derive(Default, Clone, Copy)]
pub enum AnimateDirection {
    #[default]
    Forward,
    Backward,
}

pub trait AppExt {
    fn recompile_on_interaction<Func: Resource>(
        &mut self,
        get_dummy: fn(&mut Func) -> &mut u8,
    ) -> &mut Self;
}

impl AppExt for App {
    /// Set `Func` resource as changed if there any [`INTERACTIONS`].
    fn recompile_on_interaction<Func: Resource>(
        &mut self,
        get_dummy: fn(&mut Func) -> &mut u8,
    ) -> &mut Self {
        self.add_systems(Update, move |mut func: ResMut<Func>| {
            if INTERACTIONS.read().unwrap().is_empty() == false {
                let dummy = get_dummy(&mut func);
                *dummy = dummy.wrapping_add(1);
            }
        })
    }
}
