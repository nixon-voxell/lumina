use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::action::PlayerAction;

use super::{PlayerId, PlayerInfoType, PlayerInfos};

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(PreUpdate, init_spaceships)
            .add_systems(FixedUpdate, spaceship_movement)
            .add_systems(Update, apply_damage_to_spaceships);
    }
}

fn init_spaceships(
    mut commands: Commands,
    q_spaceships: Query<(&Spaceship, Option<&Name>, Entity), Added<SourceEntity>>,
) {
    // TODO: Consider using a lookup collider.
    let collider = Collider::triangle(
        Vec2::new(-20.0, 20.0),
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, 0.0),
    );

    for (spaceship, name, spaceship_entity) in q_spaceships.iter() {
        commands.entity(spaceship_entity).insert((
            SpaceshipPhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                linear_damping: LinearDamping(spaceship.linear_damping),
                angular_damping: AngularDamping(spaceship.angular_damping),
                collider: collider.clone(),
                mass_properties: MassPropertiesBundle::new_computed(&collider, 1.0),
                ..default()
            },
            MovementStat {
                linear_acceleration: 0.0,
                linear_damping: spaceship.linear_damping,
            },
            Health {
                current: 100.0,
                max: 100.0,
            },
        ));

        debug!("Initialized Spaceship physics for {spaceship_entity:?} - ({name:?})");
    }
}

/// Perform spaceship movements which includes:
///
/// - Acceleration
/// - Deceleration
/// - Brake
/// - Steer
fn spaceship_movement(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_spaceships: Query<
        (
            &mut MovementStat,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut LinearDamping,
            &Rotation,
            &Spaceship,
        ),
        With<SourceEntity>,
    >,
    time: Res<Time>,
    player_infos: Res<PlayerInfos>,
) {
    // How fast the space ship accelerates/decelarates.
    const ACCELERATION_FACTOR: f32 = 10.0;
    const DECELERATION_FACTOR: f32 = 20.0;
    const DAMPING_FACTOR: f32 = 16.0;

    let acceleration_factor = f32::min(ACCELERATION_FACTOR * time.delta_seconds(), 1.0);
    let deceleration_factor = f32::min(DECELERATION_FACTOR * time.delta_seconds(), 1.0);
    let damping_factor = f32::min(DAMPING_FACTOR * time.delta_seconds(), 1.0);

    for (action, id) in q_actions.iter() {
        let Some(&spaceship_entity) = player_infos[PlayerInfoType::Spaceship].get(id) else {
            continue;
        };

        let Ok((
            mut movement_stat,
            mut linear,
            mut angular,
            mut linear_damping,
            rotation,
            spaceship,
        )) = q_spaceships.get_mut(spaceship_entity)
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
            let desired_angle = movement.to_angle();

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

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Spaceship {
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
pub struct SpaceshipPhysicsBundle {
    pub rigidbody: RigidBody,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub collider: Collider,
    pub mass_properties: MassPropertiesBundle,
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// Represents an event where a specific entity (target) takes damage.
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity, // The entity that will receive the damage
    pub damage: f32,    // The amount of damage to apply
}

/// Applies damage to the targeted spaceship entities based on received DamageEvents.
fn apply_damage_to_spaceships(
    mut commands: Commands,
    mut damage_evr: EventReader<DamageEvent>,
    mut q_spaceship_health: Query<(Entity, &mut Health), With<Spaceship>>,
) {
    for event in damage_evr.read() {
        if let Ok((entity, mut health)) = q_spaceship_health.get_mut(event.target) {
            health.current -= event.damage;

            if health.current <= 0.0 {
                // Handle spaceship destruction
                // TODO: Death
            }
        }
    }
}
