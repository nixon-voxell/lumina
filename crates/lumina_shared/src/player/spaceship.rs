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

        if is_boosting {
            self.energy = f32::max(0.0, self.energy - self.consumption_rate * delta_time);
            self.current_cooldown = self.cooldown_duration;
            return;
        }

        if self.current_cooldown <= 0.0 {
            self.energy = f32::min(self.max_energy, self.energy + regen_energy);
            return;
        }

        self.current_cooldown -= delta_time;
    }

    /// Attempts to activate boosting, returning true if successful.
    pub fn can_boost(&self, delta_time: f32) -> bool {
        self.energy > self.consumption_rate * delta_time
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
                linear_damping: spaceship.linear_damping,
                ..default()
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
            &mut Dash,
            &Spaceship,
            &Visibility,
        ),
        With<SourceEntity>,
    >,
    time: Res<Time>,
    player_infos: Res<PlayerInfos>,
) {
    // How fast the spaceship accelerates/decelarates.
    const ACCELERATION_FACTOR: f32 = 8.4;
    const DECELERATION_FACTOR: f32 = 12.4;
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
            mut dash,
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
        let is_dashing = action.pressed(&PlayerAction::Dash);
        let is_boosting = action.pressed(&PlayerAction::Boost);

        // Update the boost state.
        boost.update_state(time.delta_seconds(), is_boosting);

        // Handle boosting activation and determine if boosting is active.
        let boosting_active = is_boosting && boost.can_boost(time.delta_seconds());

        // // Linear damping.
        // match is_braking {
        //     true => {
        //         movement_stat
        //             .towards_linear_damping(spaceship.brake_linear_damping, damping_factor);
        //     }
        //     false => movement_stat.towards_linear_damping(spaceship.linear_damping, damping_factor),
        // }

        // Dash activation
        match (
            dash.is_dashing,
            is_dashing,
            boost.energy >= dash.energy_cost,
            dash.current_cooldown <= 0.0,
        ) {
            // Case 1: Dash is currently active
            (true, _, _, _) => {
                dash.duration -= time.delta_seconds();
                if dash.duration <= 0.0 {
                    dash.is_dashing = false;
                    dash.current_cooldown = dash.cooldown;
                    dash.direction = Vec2::ZERO;
                    movement_stat.towards_linear_damping(35.0, damping_factor);
                }
            }

            // Case 2: Dash is just starting, has energy, and not in cooldown
            (false, true, true, true) => {
                let current_direction = action
                    .clamped_axis_pair(&PlayerAction::Move)
                    .map(|axis| axis.xy())
                    .unwrap_or_else(|| Vec2::new(0.0, 0.0))
                    .normalize_or_zero();

                if current_direction != Vec2::ZERO {
                    dash.is_dashing = true;
                    dash.duration = 0.7;
                    dash.direction = current_direction;
                    boost.energy -= dash.energy_cost;
                    linear.0 = dash.direction * dash.speed;
                    movement_stat.towards_linear_damping(0.0, damping_factor);
                }
            }

            // Case 3: Dash is attempted but either in cooldown or insufficient energy
            (false, true, has_energy, not_in_cooldown) => {
                if !has_energy || !not_in_cooldown {
                    if is_moving {
                        // Calculate the current movement direction
                        let current_direction = action
                            .clamped_axis_pair(&PlayerAction::Move)
                            .map(|axis| axis.xy())
                            .unwrap_or_else(|| Vec2::ZERO)
                            .normalize_or_zero();

                        if current_direction != Vec2::ZERO {
                            // Apply acceleration to maintain velocity
                            movement_stat.towards_linear_acceleration(
                                spaceship.linear_acceleration,
                                acceleration_factor,
                                deceleration_factor,
                            );

                            // Update velocity based on the current direction and acceleration
                            linear.0 += current_direction
                                * movement_stat.linear_acceleration
                                * time.delta_seconds();
                            linear.0 = linear.0.clamp_length_max(spaceship.max_linear_speed);
                        }
                    } else {
                        // If not moving, apply braking behavior
                        movement_stat.towards_linear_acceleration(
                            0.0,
                            acceleration_factor,
                            deceleration_factor,
                        );
                    }

                    // Apply damping to smoothly reduce velocity if necessary
                    movement_stat.towards_linear_damping(spaceship.linear_damping, damping_factor);
                }
            }

            // Case 4: Normal movement (not dashing)
            (false, false, _, _) => {
                dash.direction = Vec2::ZERO;
                movement_stat.towards_linear_damping(spaceship.linear_damping, damping_factor);
            }
        }

        // Update cooldown timer
        if dash.current_cooldown > 0.0 {
            dash.current_cooldown -= time.delta_seconds();
        }

        // Linear acceleration.
        match (is_moving, is_dashing, boosting_active) {
            // Moving only.
            (true, false, false) => {
                // Apply acceleration for normal movement
                movement_stat.towards_linear_acceleration(
                    spaceship.linear_acceleration,
                    acceleration_factor,
                    deceleration_factor,
                );
            }
            // // Moving and braking.
            // (true, true, _) => movement_stat.towards_linear_acceleration(
            //     spaceship.brake_linear_acceleration,
            //     acceleration_factor,
            //     deceleration_factor,
            // ),

            // Moving and dashing.
            (true, true, _) => {
                // // Maintain dash speed without modifying acceleration
                // linear.0 = dash.direction * dash.speed;
            }
            // Moving and boosting.
            (true, false, true) => movement_stat.towards_linear_acceleration(
                spaceship.boost_linear_acceleration,
                acceleration_factor,
                deceleration_factor,
            ),
            // Not even moving, reduce speed to 0.0.
            (false, ..) => movement_stat.towards_linear_acceleration(
                0.0,
                acceleration_factor,
                deceleration_factor,
            ),
        }

        movement_stat.rotation_diff = 0.0;
        // Angular acceleration.
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

            // NOTE: Kept for posterity's sake, just in case.
            // angular.0 = rotation.angle_between(Rotation::radians(desired_angle))
            //     * spaceship.rotation_speed
            //     * time.delta_seconds();

            let prev_rotation = rotation.as_radians();
            *rotation = rotation.slerp(
                Rotation::radians(desired_angle),
                f32::min(1.0, time.delta_seconds() * spaceship.rotation_speed),
            );
            movement_stat.rotation_diff = rotation.as_radians() - prev_rotation;
        }

        let direction = Vec2::new(rotation.cos, rotation.sin);
        linear.0 += direction * movement_stat.linear_acceleration * time.delta_seconds();

        // Clamp the speed.
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
            true => *viz = Visibility::Hidden,
            false => *viz = Visibility::Inherited,
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
    rotation_diff: f32,
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

    pub fn linear_acceleration(&self) -> f32 {
        self.linear_acceleration
    }

    pub fn linear_damping(&self) -> f32 {
        self.linear_damping
    }

    pub fn rotation_diff(&self) -> f32 {
        self.rotation_diff
    }
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

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Boost {
    // Energy level.
    pub energy: f32,
    // Maximum energy level.
    pub max_energy: f32,
    // Energy regeneration rate.
    pub regen_rate: f32,
    // Energy consumption rate.
    pub consumption_rate: f32,
    // Cooldown duration in seconds before energy regenerates.
    pub cooldown_duration: f32,
    // Remaining cooldown time.
    pub current_cooldown: f32,
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component, Default)]
pub struct Dash {
    // Dash direction
    pub direction: Vec2,
    // How long the dash lasts (seconds)
    pub duration: f32,
    // Cooldown after dash (seconds)
    pub cooldown: f32,
    // Remaining cooldown time
    pub current_cooldown: f32,
    // Energy cost for dashing
    pub energy_cost: f32,
    // Dash speed
    pub speed: f32,
    // Whether the spaceship is currently dashing
    pub is_dashing: bool,
}
