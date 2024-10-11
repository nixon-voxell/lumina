use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;
use leafwing_input_manager::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

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
    q_sensors: Query<&GlobalTransform, With<Sensor>>,
    q_my_player: Query<&GlobalTransform, With<MyPlayer>>,
    mut started_evr: EventReader<CollisionStarted>,
    mut ended_evr: EventReader<CollisionEnded>,
    mut collided_effector: ResMut<CollidedEffector>,
    mut collisions: Local<HashSet<EffectorCollision>>,
) {
    let Ok(player_transform) = q_my_player.get_single() else {
        return;
    };

    // Closure for filtering collisions that only occurs between player and effector.
    let filter_collision = |collision: (Entity, Entity)| -> Option<EffectorCollision> {
        let mut player = None;
        let mut effector = None;

        if q_my_player.contains(collision.0) {
            player = Some(collision.0);
        } else if q_my_player.contains(collision.1) {
            player = Some(collision.1);
        }

        if q_sensors.contains(collision.0) {
            effector = Some(collision.0);
        } else if q_sensors.contains(collision.1) {
            effector = Some(collision.1);
        }

        if let (Some(player), Some(effector)) = (player, effector) {
            return Some(EffectorCollision { player, effector });
        }

        None
    };

    // Add and remove collisions.
    for collision in started_evr
        .read()
        .filter_map(|c| filter_collision((c.0, c.1)))
    {
        collisions.insert(collision);
    }

    for collision in ended_evr
        .read()
        .filter_map(|c| filter_collision((c.0, c.1)))
    {
        collisions.remove(&collision);
    }

    // Find the closest effector to the player.
    let mut closest_distance = f32::MAX;
    let mut closest_effector = None;

    for collision in collisions.iter() {
        let Ok(effector_translation) = q_sensors.get(collision.effector).map(|t| t.translation())
        else {
            continue;
        };

        let distance = Vec3::distance_squared(effector_translation, player_transform.translation());

        if distance < closest_distance {
            closest_distance = distance;
            closest_effector = Some(collision.effector);
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
    // mut scene: ResMut<VelystScene<EffectorPopupFunc>>,
    time: Res<Time>,
    mut animation: Local<f64>,
) {
    let Some(scope) = context.get_scope() else {
        return;
    };

    let Ok(mut popup_style) = q_popup_style.get_single_mut() else {
        return;
    };

    if let Some(entity) = **collided_effector {
        // let (camera, camera_transform) = q_camera.single();

        let Ok((effector_transform, collider, is_interactable, popup_msg)) = q_sensors.get(entity)
        else {
            return;
        };

        if collided_effector.is_changed() {
            let translation = effector_transform.translation();
            popup_style.left = Val::Px(translation.x);
            popup_style.top =
                Val::Px(translation.y + collider.shape_scaled().0.compute_local_aabb().maxs.y);

            let mut contents = Vec::new();

            if is_interactable {
                contents.push(
                    elem::context(scope.get_func_unchecked("button_popup"), |args| {
                        args.push("E");
                    })
                    .pack(),
                );
            }

            if let Some(popup_msg) = popup_msg {
                contents.push(
                    elem::context(scope.get_func_unchecked("msg_popup"), |args| {
                        args.push(popup_msg.0.clone());
                    })
                    .pack(),
                );
            }

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

        *animation = f64::min(*animation + time.delta_seconds_f64(), 1.0);
        // scene.post_process_map.insert(TypLabel::new("body"), PostPro)
    } else if func.body.is_some() {
        func.body = None;
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

/// Used to store collision information betweewn [`MyPlayer`] and [`Effector`][effector].
///
/// [effector]: crate::shared::effector::Effector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct EffectorCollision {
    pub player: Entity,
    pub effector: Entity,
}
