use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::{input::PlayerAction, LocalEntity, MovementSet};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovement>()
            .add_systems(Update, init_players)
            .add_systems(FixedUpdate, player_movement.in_set(MovementSet::Physics));
    }
}

pub fn shared_handle_player_movement(
    action_state: &ActionState<PlayerAction>,
    player_entity: Entity,
    player_movement_evw: &mut EventWriter<PlayerMovement>,
) {
    let player_movement = PlayerMovement {
        player_entity,
        is_moving: action_state.pressed(&PlayerAction::Move),
        movement: action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .map(|axis| axis.xy())
            .unwrap_or_default(),
        is_braking: action_state.pressed(&PlayerAction::Brake),
    };

    player_movement_evw.send(player_movement);
}

fn init_players(
    mut commands: Commands,
    q_players: Query<
        (&SpaceShip, Entity),
        (
            Without<Collider>,
            Or<(
                With<LocalEntity>,
                With<client::Predicted>,
                With<Replicating>,
            )>,
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

    for (space_ship, entity) in q_players.iter() {
        info!("Adding collider for {entity:?}");
        commands.entity(entity).insert((
            MassPropertiesBundle::new_computed(&collider, 1.0),
            collider.clone(),
            LinearDamping(space_ship.linear_damping),
            AngularDamping(space_ship.angular_damping),
        ));
    }
}

fn player_movement(
    mut q_movements: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut LinearDamping,
        &Rotation,
        &SpaceShip,
    )>,
    mut player_movement_evr: EventReader<PlayerMovement>,
) {
    for player_movement in player_movement_evr.read() {
        if let Ok((mut linear, mut angular, mut linear_damping, rotation, space_ship)) =
            q_movements.get_mut(player_movement.player_entity)
        {
            if player_movement.is_moving {
                let movement = player_movement.movement.normalize_or_zero();
                let desired_angle = movement.y.atan2(movement.x);

                angular.0 += rotation.angle_between(Rotation::radians(desired_angle))
                    * space_ship.angular_speed;

                let direction = Vec2::new(rotation.cos, rotation.sin);

                linear.0 += direction * space_ship.linear_speed;

                // Clamp the speed
                linear.0 = linear.clamp_length_max(space_ship.max_linear_speed);
            }

            match player_movement.is_braking {
                true => *linear_damping = LinearDamping(space_ship.brake_linear_damping),
                false => *linear_damping = LinearDamping(space_ship.linear_damping),
            }
        }
    }
}

#[derive(Event)]
pub struct PlayerMovement {
    pub player_entity: Entity,
    pub is_moving: bool,
    pub movement: Vec2,
    pub is_braking: bool,
}

#[derive(Bundle)]
pub struct ReplicatePlayerBundle {
    pub id: PlayerId,
    pub ship: SpaceShip,
    pub physics: PhysicsBundle,
}

impl ReplicatePlayerBundle {
    pub fn new(client_id: ClientId, position: Position, rotation: Rotation) -> Self {
        Self {
            id: PlayerId(client_id),
            // TODO: Make this a input parameter.
            ship: SpaceShip::assassin(),
            physics: PhysicsBundle::player()
                .with_position(position)
                .with_rotation(rotation),
        }
    }
}

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub ship: SpaceShip,
    pub physics: PhysicsBundle,
    pub local: LocalEntity,
}

impl LocalPlayerBundle {
    pub fn new(position: Position, rotation: Rotation) -> Self {
        Self {
            // TODO: Make this a input parameter.
            ship: SpaceShip::assassin(),
            physics: PhysicsBundle::player()
                .with_position(position)
                .with_rotation(rotation),
            local: LocalEntity,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
pub struct SpaceShip {
    /// Normal linear speed.
    pub linear_speed: f32,
    /// Normal angular speed.
    pub angular_speed: f32,
    /// Linear speed when boost is applied.
    pub boost_linear_speed: f32,
    /// Maximum magnitude of the linear velocity.
    pub max_linear_speed: f32,
    /// Normal linear damping.
    pub linear_damping: f32,
    /// Normal angular damping.
    pub angular_damping: f32,
    /// Linear damping when brake is applied.
    pub brake_linear_damping: f32,
}

impl SpaceShip {
    pub fn assassin() -> Self {
        Self {
            linear_speed: 50.0,
            angular_speed: 0.8,
            boost_linear_speed: 100.0,
            max_linear_speed: 400.0,
            linear_damping: 2.0,
            angular_damping: 6.0,
            brake_linear_damping: 10.0,
        }
    }
}

// pub enum SpaceShipClass {
//     Tank,
//     Assassin,
// }

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
}

impl PhysicsBundle {
    pub fn player() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            ..default()
        }
    }
}

// Builder pattern.
impl PhysicsBundle {
    pub fn with_rigidbody(mut self, rigidbody: RigidBody) -> Self {
        self.rigidbody = rigidbody;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_linear_damping(mut self, linear_damping: LinearDamping) -> Self {
        self.linear_damping = linear_damping;
        self
    }

    pub fn with_angular_damping(mut self, angular_damping: AngularDamping) -> Self {
        self.angular_damping = angular_damping;
        self
    }
}
