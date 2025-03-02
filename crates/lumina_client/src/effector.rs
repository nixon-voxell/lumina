use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use leafwing_input_manager::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::effector::{
    EffectorPopupMsg, InteractableEffector, MatchmakeEffector, SpaceshipSelectEffector,
    TesseractEffector,
};
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::typ_animation::{LabelScaleFade, TypAnimationPlugin};

use super::player::LocalPlayerInfo;

pub(super) struct EffectorPlugin;

impl Plugin for EffectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TypAnimationPlugin::<EffectorPopupFunc>::default())
            .init_resource::<CollidedEffector>()
            .add_event::<EffectorInteraction>()
            .add_systems(Startup, setup_effector_popup)
            .add_systems(FixedUpdate, collect_effector_collisions)
            .add_systems(
                Update,
                (
                    show_effector_popup,
                    interact_effector,
                    effector_trigger::<MatchmakeEffector>,
                    effector_trigger::<TesseractEffector>,
                    effector_trigger::<SpaceshipSelectEffector>,
                ),
            );
    }
}

/// Collect effector collision and place the closest result to [`CollidedEffector`].
fn collect_effector_collisions(
    q_sensors: Query<
        (&GlobalTransform, &CollidingEntities, Entity),
        (With<Sensor>, Without<InteractedEffector>),
    >,
    q_spaceship_transforms: Query<&GlobalTransform, With<SourceEntity>>,
    mut collided_effector: ResMut<CollidedEffector>,
    local_player_info: LocalPlayerInfo,
) {
    let Some((spaceship_entity, Ok(player_transform))) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .map(|entity| (entity, q_spaceship_transforms.get(entity)))
    else {
        collided_effector.set_if_neq(CollidedEffector(None));
        return;
    };

    let effectors = q_sensors
        .iter()
        .filter(|(_, colliding_entities, _)| colliding_entities.contains(&spaceship_entity));

    // Find the closest effector to the player.
    let mut closest_distance = f32::MAX;
    let mut closest_effector = None;

    for (transform, _, entity) in effectors {
        let distance =
            Vec3::distance_squared(transform.translation(), player_transform.translation());

        if distance < closest_distance {
            closest_distance = distance;
            closest_effector = Some(entity);
        }
    }

    collided_effector.set_if_neq(CollidedEffector(closest_effector));
}

fn setup_effector_popup(mut commands: Commands) {
    let id = commands.spawn(LabelScaleFade::new("body")).id();
    let sequence = commands.play_motion(
        Action::new_f32lerp(id, 0.0, 1.0, |label: &mut LabelScaleFade| &mut label.time)
            .with_ease(ease::cubic::ease_in_out)
            .animate(0.2),
    );

    commands.spawn((
        SequencePlayerBundle::from_sequence(sequence),
        EffectorPopupAnimation,
    ));
}

/// Show and animate the effector popup.
fn show_effector_popup(
    q_sensors: Query<
        (
            &GlobalTransform,
            &Collider,
            Has<InteractableEffector>,
            Option<&EffectorPopupMsg>,
        ),
        With<Sensor>,
    >,
    mut q_seq_player: Query<
        (&mut SequencePlayer, &SequenceController),
        With<EffectorPopupAnimation>,
    >,
    mut q_style: Query<&mut Style, With<VelystSceneTag<EffectorPopupFunc>>>,
    collided_effector: Res<CollidedEffector>,
    mut func: ResMut<EffectorPopupFunc>,
    // The effector entity that ui is currently positioned at.
    mut curr_effector: Local<Option<Entity>>,
) {
    let Ok(mut popup_style) = q_style.get_single_mut() else {
        return;
    };
    let (mut player, controller) = q_seq_player.single_mut();

    if *curr_effector != **collided_effector {
        // Hide the effector on change.
        player.time_scale = -1.0;
    } else if curr_effector.is_some() {
        // Show the effector.
        player.time_scale = 1.0;
    }

    if controller.curr_time() < f32::EPSILON {
        // Update target effector when successfully hidden.
        *curr_effector = **collided_effector;

        // Setup new effector if exists.
        if let Some((effector_transform, collider, is_interactable, popup_msg)) =
            curr_effector.and_then(|entity| q_sensors.get(entity).ok())
        {
            // Set translation of the ui.
            let translation = effector_transform.translation();
            popup_style.left = Val::Px(translation.x);
            popup_style.top =
                Val::Px(translation.y + collider.shape_scaled().0.compute_local_aabb().maxs.y);

            // Show which button to press if it's interactable.
            match is_interactable {
                // TODO: Keep track of last input device.
                true => func.button = Some("E"),
                false => func.button = None,
            }

            // Show popup message if available.
            match popup_msg {
                Some(popup_msg) => func.message = Some(popup_msg.0.clone()),
                None => func.message = None,
            }
        }
    }
}

/// Track interaction from local player when it is above the effector.
fn interact_effector(
    mut commands: Commands,
    q_effectors: Query<(&InteractableEffector, Entity), Without<InteractedEffector>>,
    q_actions: Query<&ActionState<PlayerAction>, With<SourceEntity>>,
    mut effector_interaction_evw: EventWriter<EffectorInteraction>,
    collided_effector: Res<CollidedEffector>,
    time: Res<Time>,
    mut func: ResMut<EffectorPopupFunc>,
    mut accumulation: Local<f32>,
    local_player_info: LocalPlayerInfo,
) {
    if collided_effector.is_changed() {
        *accumulation = 0.0;
    }

    if func.has_content() == false {
        return;
    }

    let Some(action) = local_player_info
        .get(PlayerInfoType::Action)
        .and_then(|e| q_actions.get(e).ok())
    else {
        return;
    };

    let Some((effector, entity)) = collided_effector.and_then(|e| q_effectors.get(e).ok()) else {
        return;
    };

    // Prevent division by zero.
    let required_duration = f32::max(effector.interact_duration, f32::EPSILON);

    if action.pressed(&PlayerAction::Interact) {
        *accumulation = f32::min(*accumulation + time.delta_seconds(), required_duration);
    } else if action.released(&PlayerAction::Interact) {
        *accumulation = f32::max(*accumulation - time.delta_seconds(), 0.0);
    }

    if *accumulation >= required_duration {
        // Perform interaction
        effector_interaction_evw.send(EffectorInteraction(entity));
        commands.entity(entity).insert(InteractedEffector);
        *accumulation = 0.0;
    }

    func.button_progress = ease::cubic::ease_in_out(*accumulation / required_duration) as f64;
}

/// Trigger event when an effector has been interacted.
pub(super) fn effector_trigger<E: Event + Clone>(
    mut commands: Commands,
    q_effector: Query<&E>,
    mut evr_interaction: EventReader<EffectorInteraction>,
) {
    for effector in evr_interaction.read() {
        let entity = **effector;
        if let Ok(e) = q_effector.get(entity) {
            commands.trigger_targets(e.clone(), entity);
        }
    }
}

/// Collided effector that is closest to the player.
#[derive(Resource, Default, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub(super) struct CollidedEffector(pub Option<Entity>);

/// Event sent after a successful interaction with an effector.
#[derive(Event, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub(super) struct EffectorInteraction(pub Entity);

/// Tag component for an [`InteractableEffector`] that has been successfully interacted.
#[derive(Component, Default)]
pub(super) struct InteractedEffector;

#[derive(Component)]
struct EffectorPopupAnimation;
