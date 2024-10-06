use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::{input::PlayerAction, MovementSet};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovement>()
            .add_systems(PostUpdate, init_players)
            .add_systems(FixedUpdate, player_movement.in_set(MovementSet::Physics));
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

fn init_players(
    mut commands: Commands,
    q_players: Query<
        Entity,
        (
            With<SpaceShip>,
            Without<Collider>,
            Or<(With<Replicating>, With<client::Predicted>)>,
        ),
    >,
) {
    if q_players.is_empty() {
        return;
    }

    let collider = Collider::triangle(
        Vec2::new(-20.0, 20.0),
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, 0.0),
    );

    for entity in q_players.iter() {
        info!("Adding collider for {entity:?}");
        commands.entity(entity).insert((
            MassPropertiesBundle::new_computed(&collider, 1.0),
            collider.clone(),
        ));
    }
}

fn player_movement(
    mut q_movements: Query<(&mut LinearVelocity, &mut AngularVelocity, &Rotation), With<PlayerId>>,
    mut player_movement_evr: EventReader<PlayerMovement>,
) {
    const MOVEMENT_SPEED: f32 = 50.0;
    // const ROTATION_SPEED: f32 = 0.4;
    const ROTATION_SPEED: f32 = 1.0;
    const MAX_SPEED: f32 = 400.0;

    for player_movement in player_movement_evr.read() {
        if let Ok((mut linear, mut angular, rotation)) =
            q_movements.get_mut(player_movement.player_entity)
        {
            let movement = player_movement.movement.normalize_or_zero();
            let desired_angle = movement.y.atan2(movement.x);

            angular.0 += rotation.angle_between(Rotation::radians(desired_angle)) * ROTATION_SPEED;

            let direction = Vec2::new(rotation.cos, rotation.sin);

            linear.0 += direction * MOVEMENT_SPEED;

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
    pub ship: SpaceShip,
    pub position: Position,
    pub rotation: Rotation,
    pub physics_bundle: PhysicsBundle,
}

impl ReplicatePlayerBundle {
    pub fn new(client_id: ClientId, position: Vec2, rotation: f32) -> Self {
        Self {
            id: PlayerId(client_id),
            ship: SpaceShip,
            position: Position::new(position),
            rotation: Rotation::radians(rotation),
            physics_bundle: PhysicsBundle::player(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct SpaceShip;

// pub enum SpaceShipClass {
//     Tank,
//     Assassin,
// }

// TODO: Make a config which gets replicated on the client side...
// TODO: Create a shared system that converts player config into actual components
// pub struct PlayerConfig {
//     pub density: f32,
// }

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
}

impl PhysicsBundle {
    pub fn player() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            linear_damping: LinearDamping(2.0),
            angular_damping: AngularDamping(6.0),
        }
    }
}
