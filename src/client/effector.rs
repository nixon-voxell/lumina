use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;
use velyst::typst_element::prelude::*;

use crate::ui::effector_popup::EffectorPopupFunc;

use super::camera::GameCamera;
use super::player::MyPlayer;

pub(super) struct EffectorPlugin;

impl Plugin for EffectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, collect_effector_collisions);
    }
}

fn collect_effector_collisions(
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    q_sensors: Query<&GlobalTransform, With<Sensor>>,
    q_my_player: Query<&GlobalTransform, With<MyPlayer>>,
    mut started_evr: EventReader<CollisionStarted>,
    mut ended_evr: EventReader<CollisionEnded>,
    mut func: ResMut<EffectorPopupFunc>,
    mut collisions: Local<HashSet<EffectorCollision>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

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
    let mut closest_effector_translation = None;
    let mut closest_distance = f32::MAX;

    for collision in collisions.iter() {
        let Ok(effector_translation) = q_sensors.get(collision.effector).map(|t| t.translation())
        else {
            continue;
        };

        let distance = Vec3::distance_squared(effector_translation, player_transform.translation());

        if distance < closest_distance {
            closest_distance = distance;
            closest_effector_translation = Some(effector_translation);
        }
    }

    if let Some(viewport_coordinate) =
        closest_effector_translation.and_then(|t| camera.world_to_viewport(camera_transform, t))
    {
        println!("got viewport coordinate");
        func.x = viewport_coordinate.x as f64;
        func.y = viewport_coordinate.y as f64;
        func.body = Some(text::TextElem::new(format!("{}", viewport_coordinate).into()).pack());
    }
}

/// Used to store collision information betweewn [`MyPlayer`] and [`Effector`][effector].
///
/// [effector]: crate::shared::effector::Effector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct EffectorCollision {
    pub player: Entity,
    pub effector: Entity,
}
