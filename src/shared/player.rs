use avian2d::prelude::*;
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::utils::EntityRoomId;

use super::input::{InputTarget, PlayerAction};
use super::LocalEntity;

pub(super) struct PlayerPlugin;

pub mod weapon;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(weapon::WeaponPlugin);

        app.init_resource::<PlayerInfos>()
            .add_systems(Update, init_networked_inputs)
            .add_systems(FixedUpdate, player_movement);
    }
}

/// Set [`InputTarget`] for networked inputs.
fn init_networked_inputs(
    mut commands: Commands,
    q_actions: Query<
        (&PlayerId, Entity),
        (
            Or<(
                // Client inputs
                With<client::Predicted>,
                // Server inputs
                With<server::SyncTarget>,
            )>,
            Added<ActionState<PlayerAction>>,
            Without<LocalEntity>,
        ),
    >,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (id, entity) in q_actions.iter() {
        if let Some(info) = player_infos.get_mut(&id.0) {
            commands
                .entity(entity)
                .insert(InputTarget::new(info.player));

            info.input = Some(entity);
            info!("Initialized input for {:?}", id);
        }
    }
}

/// Perform player movement which includes:
///
/// - Acceleration
/// - Deceleration
/// - Brake
/// - Steer
fn player_movement(
    q_actions: Query<(&ActionState<PlayerAction>, &InputTarget)>,
    mut q_spaceships: Query<(
        &mut MovementStat,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut LinearDamping,
        &Rotation,
        &SpaceShip,
    )>,
    time: Res<Time>,
) {
    // How fast the space ship accelerates/decelarates.
    const ACCELERATION_FACTOR: f32 = 10.0;
    const DECELERATION_FACTOR: f32 = 20.0;
    const DAMPING_FACTOR: f32 = 16.0;

    let acceleration_factor = f32::min(ACCELERATION_FACTOR * time.delta_seconds(), 1.0);
    let deceleration_factor = f32::min(DECELERATION_FACTOR * time.delta_seconds(), 1.0);
    let damping_factor = f32::min(DAMPING_FACTOR * time.delta_seconds(), 1.0);

    for (action, target) in q_actions.iter() {
        let player_entity = **target;

        let Ok((
            mut movement_stat,
            mut linear,
            mut angular,
            mut linear_damping,
            rotation,
            spaceship,
        )) = q_spaceships.get_mut(player_entity)
        else {
            continue;
        };

        let is_moving = action.pressed(&PlayerAction::Move);
        let is_braking = action.pressed(&PlayerAction::Brake);
        let is_boosting = action.pressed(&PlayerAction::Boost);

        // Linear damping
        match is_braking {
            true => {
                movement_stat.towards_linear_damping(spaceship.brake_linear_damping, damping_factor)
            }
            false => movement_stat.towards_linear_damping(spaceship.linear_damping, damping_factor),
        }

        // Linear acceleration
        match (is_moving, is_braking, is_boosting) {
            // Moving only
            (true, false, false) => movement_stat.towards_linear_acceleration(
                spaceship.linear_acceleration,
                acceleration_factor,
                deceleration_factor,
            ),
            // Moving and braking
            (true, true, _) => movement_stat.towards_linear_acceleration(
                spaceship.brake_linear_acceleration,
                acceleration_factor,
                deceleration_factor,
            ),
            // Moving and boosting
            (true, false, true) => movement_stat.towards_linear_acceleration(
                spaceship.boost_linear_acceleration,
                acceleration_factor,
                deceleration_factor,
            ),
            // Not even moving, reduce speed to 0.0
            (false, ..) => movement_stat.towards_linear_acceleration(
                0.0,
                acceleration_factor,
                deceleration_factor,
            ),
        }

        // Angular acceleration
        if is_moving {
            movement_stat.linear_acceleration = FloatExt::lerp(
                movement_stat.linear_acceleration,
                spaceship.linear_acceleration,
                acceleration_factor,
            );

            let movement = action
                .clamped_axis_pair(&PlayerAction::Move)
                .map(|axis| axis.xy())
                .unwrap_or_default()
                .normalize_or_zero();
            let desired_angle = movement.y.atan2(movement.x);

            angular.0 += rotation.angle_between(Rotation::radians(desired_angle))
                * spaceship.angular_acceleration
                * time.delta_seconds();
        }

        let direction = Vec2::new(rotation.cos, rotation.sin);
        linear.0 += direction * movement_stat.linear_acceleration * time.delta_seconds();

        // Clamp the speed
        linear.0 = linear.clamp_length_max(spaceship.max_linear_speed);
        linear_damping.0 = movement_stat.linear_damping;
    }
}

#[derive(Bundle)]
pub struct ReplicatePlayerBundle {
    pub spaceship: SpaceShip,
    pub id: PlayerId,
}

impl ReplicatePlayerBundle {
    pub fn new(client_id: ClientId, spaceship: SpaceShip) -> Self {
        Self {
            spaceship,
            id: PlayerId(client_id),
        }
    }
}

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub spaceship: SpaceShip,
    pub local_entity: LocalEntity,
    pub local_player: LocalPlayer,
}

impl LocalPlayerBundle {
    pub fn new(spaceship: SpaceShip) -> Self {
        Self {
            spaceship,
            local_entity: LocalEntity,
            local_player: LocalPlayer,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
pub struct SpaceShip {
    /// Normal linear acceleration.
    pub linear_acceleration: f32,
    /// Normal angular acceleration.
    pub angular_acceleration: f32,
    /// Linear acceleration when boost is applied.
    pub boost_linear_acceleration: f32,
    /// Linear acceleration when brake is applied.
    pub brake_linear_acceleration: f32,
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
            linear_acceleration: 800.0,
            angular_acceleration: 30.0,
            boost_linear_acceleration: 4000.0,
            brake_linear_acceleration: 600.0,
            max_linear_speed: 1000.0,
            linear_damping: 2.0,
            angular_damping: 6.0,
            brake_linear_damping: 4.0,
        }
    }
}

impl Component for SpaceShip {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let entity_ref = world.entity(entity);

            if entity_ref.contains::<Replicated>() {
                return;
            }

            let spaceship = entity_ref.get::<Self>().unwrap();
            // TODO: Consider using a lookup collider.
            let collider = Collider::triangle(
                Vec2::new(-10.0, 10.0),
                Vec2::new(-10.0, -10.0),
                Vec2::new(10.0, 0.0),
            );

            let bundle = (
                MassPropertiesBundle::new_computed(&collider, 1.0),
                collider.clone(),
                PhysicsBundle {
                    rigidbody: RigidBody::Dynamic,
                    linear_damping: LinearDamping(spaceship.linear_damping),
                    angular_damping: AngularDamping(spaceship.angular_damping),
                    ..default()
                },
                MovementStat {
                    linear_acceleration: 0.0,
                    linear_damping: spaceship.linear_damping,
                },
            );

            world.commands().entity(entity).insert(bundle);

            info!("Initialized spaceship for {entity:?}");
        });
    }
}

/// The movement stat for a spaceship.
#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
pub struct MovementStat {
    linear_acceleration: f32,
    linear_damping: f32,
}

impl MovementStat {
    pub fn towards_linear_acceleration(
        &mut self,
        target: f32,
        acceleration_factor: f32,
        deceleration_factor: f32,
    ) {
        let factor = match self.linear_acceleration.total_cmp(&target) {
            std::cmp::Ordering::Less => acceleration_factor,
            std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Greater => deceleration_factor,
        };
        self.linear_acceleration = FloatExt::lerp(self.linear_acceleration, target, factor);
    }

    pub fn towards_linear_damping(&mut self, target: f32, factor: f32) {
        self.linear_damping = FloatExt::lerp(self.linear_damping, target, factor);
    }

    // pub fn linear_acceleration(&self) -> f32 {
    //     self.linear_acceleration
    // }

    // pub fn linear_damping(&self) -> f32 {
    //     self.linear_damping
    // }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
}

/// The player that the local client is controlling.
#[derive(Component, Default)]
pub struct LocalPlayer;

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct PlayerInfos(HashMap<ClientId, PlayerInfo>);

#[derive(Debug)]
pub struct PlayerInfo {
    /// The lobby entity.
    pub lobby: Entity,
    /// The player entity ([`crate::shared::player::SpaceShip`]).
    pub player: Entity,
    /// The input entity that is controlling [`Self::player`].
    pub input: Option<Entity>,
}

impl PlayerInfo {
    /// Returns the [`server::RoomId`] of the lobby.
    pub fn room_id(&self) -> server::RoomId {
        self.lobby.room_id()
    }
}
