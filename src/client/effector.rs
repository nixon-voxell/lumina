use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_motiongfx::prelude::ease;
use leafwing_input_manager::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;
use velyst::typst_vello;

use crate::shared::effector::{EffectorPopupMsg, InteractableEffector};
use crate::shared::input::PlayerAction;
use crate::ui::effector_popup::{EffectorPopupFunc, EffectorPopupUi};

use super::player::MyPlayer;

pub(super) struct EffectorPlugin;

impl Plugin for EffectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollidedEffector>()
            .add_systems(FixedUpdate, collect_effector_collisions)
            .add_systems(Update, (show_effector_popup, interact_effector));
    }
}

/// Collect effector collision and place the closest result to [`CollidedEffector`].
fn collect_effector_collisions(
    q_sensors: Query<(&GlobalTransform, &CollidingEntities, Entity), With<Sensor>>,
    q_my_player: Query<(&GlobalTransform, Entity), With<MyPlayer>>,
    mut collided_effector: ResMut<CollidedEffector>,
) {
    let Ok((player_transform, player_entity)) = q_my_player.get_single() else {
        return;
    };

    let effectors = q_sensors
        .iter()
        .filter(|(_, colliding_entities, _)| colliding_entities.contains(&player_entity));

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

fn show_effector_popup(
    context: TypstContext<EffectorPopupUi>,
    q_sensors: Query<
        (
            &GlobalTransform,
            &Collider,
            Has<InteractableEffector>,
            Option<&EffectorPopupMsg>,
        ),
        With<Sensor>,
    >,
    mut q_popup_style: Query<&mut Style, With<VelystSceneTag<EffectorPopupFunc>>>,
    collided_effector: Res<CollidedEffector>,
    mut func: ResMut<EffectorPopupFunc>,
    mut scene: ResMut<VelystScene<EffectorPopupFunc>>,
    time: Res<Time>,
    // Animtion time for the ui.
    mut animation: Local<f32>,
    // The effector entity that ui is currently positioned at.
    mut curr_effector: Local<Option<Entity>>,
) {
    let Some(scope) = context.get_scope() else {
        return;
    };

    let Ok(mut popup_style) = q_popup_style.get_single_mut() else {
        return;
    };

    const ANIMATION_SPEED: f32 = 4.0;
    let animation_delta = time.delta_seconds() * ANIMATION_SPEED;

    let mut effector_changed = false;

    if *curr_effector != **collided_effector {
        // Hide the effector on change.
        *animation = f32::max(*animation - animation_delta, 0.0);

        // Update target effector when successfully hidden.
        if *animation <= 0.0 {
            *curr_effector = **collided_effector;
            effector_changed = true;
        }
    } else if curr_effector.is_some() {
        // Show the effector.
        *animation = f32::min(*animation + animation_delta, 1.0);
    }

    if effector_changed {
        if let Some((effector_transform, collider, is_interactable, popup_msg)) =
            curr_effector.and_then(|entity| q_sensors.get(entity).ok())
        {
            // Set translation of the ui.
            let translation = effector_transform.translation();
            popup_style.left = Val::Px(translation.x);
            popup_style.top =
                Val::Px(translation.y + collider.shape_scaled().0.compute_local_aabb().maxs.y);

            let mut contents = Vec::new();

            // Show which button to press if it's interactable.
            if is_interactable {
                contents.push(
                    elem::context(scope.get_func_unchecked("button_popup"), |args| {
                        args.push("E");
                    })
                    .pack(),
                );
            }

            // Show popup message if available.
            if let Some(popup_msg) = popup_msg {
                contents.push(
                    elem::context(scope.get_func_unchecked("msg_popup"), |args| {
                        args.push(popup_msg.0.clone());
                    })
                    .pack(),
                );
            }

            // Stack ui elements together from left to right.
            let stack = elem::stack(
                contents
                    .iter()
                    .map(|c| layout::StackChild::Block(c.clone()))
                    .collect::<Vec<_>>(),
            )
            .with_dir(layout::Dir::LTR)
            .with_spacing(Some(layout::Spacing::Rel(Abs::pt(10.0).rel())));

            func.body = Some(stack.pack());
        }
    }

    // Do not render ui when there is no active effector and it has been hidden through animation.
    if *animation <= 0.0 && curr_effector.is_none() {
        func.body = None;
    }

    if func.body.is_some() {
        let label = TypLabel::new("body");

        if let Some(group_index) = scene.query(label).and_then(|g| g.first()) {
            // Preserve original group transform.
            let transform = scene.get_group(*group_index).transform();
            // Ease time.
            let t = ease::cubic::ease_in_out(*animation);

            scene.post_process_map.insert(
                label,
                typst_vello::PostProcess {
                    transform: Some(transform.pre_scale(f64::lerp(0.5, 1.0, t as f64))),
                    layer: Some(typst_vello::Layer {
                        alpha: t,
                        ..default()
                    }),
                    ..default()
                },
            );
        }
    }
}

fn interact_effector(
    q_effectors: Query<&InteractableEffector>,
    action: Res<ActionState<PlayerAction>>,
    collided_effector: Res<CollidedEffector>,
    time: Res<Time>,
    mut accumulation: Local<f32>,
) {
    if collided_effector.is_changed() {
        *accumulation = 0.0;
    }

    let Some(effector) = collided_effector.and_then(|e| q_effectors.get(e).ok()) else {
        return;
    };

    if action.pressed(&PlayerAction::Interact) {
        *accumulation += time.delta_seconds();

        if *accumulation >= effector.required_accumulation {
            // Perform interaction
        }
    } else if action.released(&PlayerAction::Interact) {
        *accumulation -= time.delta_seconds();
    }
}

/// Collided effector that is closest to the player.
#[derive(Resource, Default, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub(super) struct CollidedEffector(pub Option<Entity>);
