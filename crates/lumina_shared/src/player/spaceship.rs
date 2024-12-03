use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::action::PlayerAction;
use crate::health::Health;

use super::{PlayerId, PlayerInfoType, PlayerInfos};

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_spaceships)
            .add_systems(FixedUpdate, spaceship_movement)
            .add_systems(PostUpdate, spaceship_health);
    }
}

impl Boost {
    /// Handles state transitions and logic based on time progression.
    pub fn update_state(&mut self, delta_time: f32, is_boosting: bool) {
        let regen_energy = self.regen_rate * delta_time;
        let new_energy = (self.energy + regen_energy).min(self.max_energy);
        let new_cooldown = self.current_cooldown - delta_time;

        match self.state {
            BoostState::Boosting => {
                // Transition to Idle if not boosting or energy is depleted
                if !is_boosting || self.energy <= f32::EPSILON {
                    self.state = BoostState::Idle;
                    self.energy = new_energy; // Regenerate energy
                }
            }
            BoostState::Cooldown => {
                self.current_cooldown = new_cooldown;
                // Transition to Idle if cooldown is complete
                if self.current_cooldown <= 0.0 {
                    self.state = BoostState::Idle;
                    self.energy = new_energy; // Regenerate energy
                } else {
                    // Regenerate energy during cooldown
                    self.energy = new_energy;
                }
            }
            BoostState::Idle => {
                // Always regenerate energy while idle
                self.energy = new_energy;
                // Transition to Boosting if boost is activated and there's enough energy
                if is_boosting && self.energy > f32::EPSILON {
                    self.state = BoostState::Boosting;
                }
            }
        }
    }

    /// Attempts to activate boosting, returning true if successful.
    pub fn try_boost(&mut self, delta_time: f32) -> bool {
        if self.state == BoostState::Idle && self.energy >= self.consumption_rate * delta_time {
            // Start boosting
            self.state = BoostState::Boosting;
            self.energy =
                (self.energy - self.consumption_rate * delta_time).clamp(0.0, self.max_energy);
            return true;
        }

        // Remain in boosting if enough energy is present
        if self.state == BoostState::Boosting {
            self.energy =
                (self.energy - self.consumption_rate * delta_time).clamp(0.0, self.max_energy);
            return true;
        }

        false
    }
}

fn init_spaceships(
    mut commands: Commands,
    q_spaceships: Query<(&Spaceship, Entity), Added<SourceEntity>>,
) {
    // TODO: Consider using a lookup collider.
    let collider = Collider::triangle(
        Vec2::new(-20.0, 20.0),
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, 0.0),
    );

    for (spaceship, spaceship_entity) in q_spaceships.iter() {
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
        ));

        debug!("Initialized Spaceship physics for {spaceship_entity})");
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
            &mut LinearDamping,
            &mut Rotation,
            &mut Boost,
            &Spaceship,
            &Visibility,
        ),
        With<SourceEntity>,
    >,
    time: Res<Time>,
    player_infos: Res<PlayerInfos>,
) {
    // How fast the spaceship accelerates/decelarates.
    const ACCELERATION_FACTOR: f32 = 10.0;
    const DECELERATION_FACTOR: f32 = 20.0;
    const DAMPING_FACTOR: f32 = 16.0;

    let acceleration_factor = f32::min(1.0, ACCELERATION_FACTOR * time.delta_seconds());
    let deceleration_factor = f32::min(1.0, DECELERATION_FACTOR * time.delta_seconds());
    let damping_factor = f32::min(1.0, DAMPING_FACTOR * time.delta_seconds());

    for (action, id) in q_actions.iter() {
        let Some(&spaceship_entity) = player_infos[PlayerInfoType::Spaceship].get(id) else {
            continue;
        };

        let Ok((
            mut movement_stat,
            mut linear,
            mut linear_damping,
            mut rotation,
            mut boost,
            spaceship,
            viz,
        )) = q_spaceships.get_mut(spaceship_entity)
        else {
            continue;
        };

        // Cannot move hidden spaceship.
        if viz == Visibility::Hidden {
            continue;
        }

        let is_moving = action.pressed(&PlayerAction::Move);
        let is_braking = action.pressed(&PlayerAction::Brake);
        let is_boosting = action.pressed(&PlayerAction::Boost);

        // Update the boost state
        boost.update_state(time.delta_seconds(), is_boosting);

        // Handle boosting activation and determine if boosting is active
        let boosting_active = if is_boosting && boost.try_boost(time.delta_seconds()) {
            info!("Boost activated by right-click");
            true
        } else {
            if is_boosting {
                info!("Boost failed to activate (insufficient energy or not in Idle state)");
            }
            false
        };

        // Linear damping
        match is_braking {
            true => {
                movement_stat
                    .towards_linear_damping(spaceship.brake_linear_damping, damping_factor);
            }
            false => movement_stat.towards_linear_damping(spaceship.linear_damping, damping_factor),
        }

        // Linear acceleration
        match (is_moving, is_braking, boosting_active) {
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

            *rotation = rotation.slerp(
                Rotation::radians(desired_angle),
                f32::min(1.0, time.delta_seconds() * spaceship.rotation_speed),
            );
        }

        let direction = Vec2::new(rotation.cos, rotation.sin);
        linear.0 += direction * movement_stat.linear_acceleration * time.delta_seconds();

        // Clamp the speed
        linear.0 = linear.clamp_length_max(spaceship.max_linear_speed);
        linear_damping.0 = movement_stat.linear_damping;
    }
}

pub(super) fn spaceship_health(
    mut q_spaceships: Query<
        (&Health, &mut Visibility),
        (Changed<Health>, With<Spaceship>, With<SourceEntity>),
    >,
) {
    for (health, mut viz) in q_spaceships.iter_mut() {
        info!("{health:?}");
        match **health <= 0.0 {
            true => {
                *viz = Visibility::Hidden;
            }
            false => {
                *viz = Visibility::Inherited;
            }
        }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Spaceship {
    /// Normal linear acceleration.
    pub linear_acceleration: f32,
    /// Rotation speed of the spaceship.
    pub rotation_speed: f32,
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
            std::cmp::Ordering::Equal => {
                return;
            }
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

/// Represents the state of the Boost system.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub enum BoostState {
    Idle,     // Not boosting
    Boosting, // Actively boosting
    Cooldown, // In cooldown period
}

impl Default for BoostState {
    fn default() -> Self {
        BoostState::Idle // Default state
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Boost {
    // Energy level
    pub energy: f32,
    // Maximum energy level
    pub max_energy: f32,
    // Energy regeneration rate
    pub regen_rate: f32,
    // Energy consumption rate
    pub consumption_rate: f32,
    // Cooldown time in seconds
    pub cooldown_duration: f32,
    // Remaining cooldown time
    pub current_cooldown: f32,
    // Current state of the boost system
    pub state: BoostState,
}
