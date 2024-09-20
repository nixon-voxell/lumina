use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::{input::PlayerAction, FixedSet};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovement>()
            .add_systems(FixedUpdate, player_movement.in_set(FixedSet::Main));
    }
}

pub fn shared_handle_player_movement(
    action_state: &ActionState<PlayerAction>,
    player_entity: Entity,
    player_movement_evw: &mut EventWriter<PlayerMovement>,
) {
    if action_state.pressed(&PlayerAction::Move) {
        let Some(movement) = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .map(|axis| axis.xy())
        else {
            return;
        };

        player_movement_evw.send(PlayerMovement {
            movement,
            player_entity,
        });
    }
}

fn player_movement(
    mut q_movements: Query<(&mut LinearVelocity, &mut AngularVelocity, &Rotation), With<PlayerId>>,
    mut player_movement_evr: EventReader<PlayerMovement>,
) {
    const THURSTER: f32 = 20.0;
    const MAX_SPEED: f32 = 200.0;
    for player_movement in player_movement_evr.read() {
        if let Ok((mut linear, mut angular, rotation)) =
            q_movements.get_mut(player_movement.player_entity)
        {
            let movement = player_movement.movement.normalize_or_zero();
            let desired_angle = movement.y.atan2(movement.x);

            angular.0 = rotation.angle_between(Rotation::radians(desired_angle));

            linear.0 += movement * THURSTER;

            // Clamp the speed
            linear.0 = linear.clamp_length_max(MAX_SPEED);
        }
    }
}

#[derive(Event)]
pub struct PlayerMovement {
    pub movement: Vec2,
    pub player_entity: Entity,
}

#[derive(Bundle)]
pub struct ReplicatePlayerBundle {
    pub id: PlayerId,
    pub position: Position,
    pub physics_bundle: PhysicsBundle,
}

impl ReplicatePlayerBundle {
    pub fn new(client_id: ClientId, position: Vec2) -> Self {
        Self {
            id: PlayerId(client_id),
            position: Position::new(position),
            physics_bundle: PhysicsBundle::player(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub collider: Collider,
    pub rigidbody: RigidBody,
}

impl PhysicsBundle {
    pub fn player() -> Self {
        Self {
            collider: Collider::rectangle(1.0, 1.0),
            rigidbody: RigidBody::Dynamic,
        }
    }
}
