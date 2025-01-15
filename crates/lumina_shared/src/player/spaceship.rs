use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use std::collections::HashMap;

use super::{PlayerId, PlayerInfoType, PlayerInfos};
use crate::action::PlayerAction;
use crate::health::Health;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SpaceshipMovementSet {
    Input,
    State,
    Physics,
}

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DashEvent>()
            .configure_sets(
                FixedUpdate,
                (
                    SpaceshipMovementSet::Input,
                    SpaceshipMovementSet::State,
                    SpaceshipMovementSet::Physics,
                )
                    .chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    handle_movement_input.in_set(SpaceshipMovementSet::Input),
                    emit_dash_events.in_set(SpaceshipMovementSet::Input),
                    handle_dash_events.in_set(SpaceshipMovementSet::State),
                    handle_boost.in_set(SpaceshipMovementSet::State),
                    apply_movement.in_set(SpaceshipMovementSet::Physics),
                ),
            )
            .add_systems(PreUpdate, init_spaceships)
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
            DesiredVelocity::default(),
        ));

        debug!("Initialized Spaceship physics for {spaceship_entity})");
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DesiredVelocity(pub Vec2);

fn handle_movement_input(
    time: Res<Time>,
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_spaceships: Query<(
        &mut MovementStat,
        &mut Rotation,
        &Spaceship,
        &Boost,
        &mut DesiredVelocity,
    )>,
    player_infos: Res<PlayerInfos>,
) {
    const ACCELERATION_FACTOR: f32 = 8.4;
    const DECELERATION_FACTOR: f32 = 12.4;

    let delta = time.delta_seconds();
    let accel_factor = f32::min(1.0, ACCELERATION_FACTOR * delta);
    let decel_factor = f32::min(1.0, DECELERATION_FACTOR * delta);

    for (action, id) in q_actions.iter() {
        let Some(&entity) = player_infos[PlayerInfoType::Spaceship].get(id) else {
            continue;
        };

        let Ok((mut movement_stat, mut rotation, spaceship, boost, mut desired_velocity)) =
            q_spaceships.get_mut(entity)
        else {
            continue;
        };

        let is_moving = action.pressed(&PlayerAction::Move);

        // Update linear acceleration based on movement and boost state
        let target_acceleration = if is_moving {
            if boost.is_boosting {
                spaceship.boost_linear_acceleration
            } else {
                spaceship.linear_acceleration
            }
        } else {
            0.0
        };

        movement_stat.towards_linear_acceleration(target_acceleration, accel_factor, decel_factor);

        // Handle rotation if moving
        if is_moving {
            if let Some(movement) = action
                .clamped_axis_pair(&PlayerAction::Move)
                .map(|axis| axis.xy())
            {
                if movement != Vec2::ZERO {
                    let movement_normalized = movement.normalize_or_zero();
                    let desired_angle = movement_normalized.to_angle();
                    let prev_rotation = rotation.as_radians();
                    *rotation = rotation.slerp(
                        Rotation::radians(desired_angle),
                        f32::min(1.0, delta * spaceship.rotation_speed),
                    );
                    movement_stat.rotation_diff = rotation.as_radians() - prev_rotation;
                }
            }
        } else {
            movement_stat.rotation_diff = 0.0;
        }

        // Update desired velocity based on current rotation
        let angle = rotation.as_radians();
        let (cos, sin) = (angle.cos(), angle.sin());
        desired_velocity.0 = Vec2::new(cos, sin) * movement_stat.linear_acceleration() * 1.5;
    }
}
// Dash Section
#[derive(Event)]
pub struct DashEvent {
    pub entity: Entity,
    pub direction: Vec2,
}

fn emit_dash_events(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    player_infos: Res<PlayerInfos>,
    mut event_writer: EventWriter<DashEvent>,
) {
    for (action, id) in q_actions.iter() {
        if action.just_pressed(&PlayerAction::Dash) {
            if let Some(&entity) = player_infos[PlayerInfoType::Spaceship].get(id) {
                let direction = action
                    .clamped_axis_pair(&PlayerAction::Move)
                    .map(|axis| axis.xy())
                    .unwrap_or_default()
                    .normalize_or_zero();

                if direction != Vec2::ZERO {
                    event_writer.send(DashEvent { entity, direction });
                }
            }
        }
    }
}

fn handle_dash_events(
    mut events: EventReader<DashEvent>,
    mut q_spaceships: Query<(
        &mut Dash,
        &mut Boost,
        &mut MovementStat,
        &mut LinearVelocity,
    )>,
    time: Res<Time>,
) {
    const DASH_DURATION: f32 = 0.4;
    const DAMPING_FACTOR: f32 = 16.0;

    let delta = time.delta_seconds();
    let damp_factor = f32::min(1.0, DAMPING_FACTOR * delta);

    // Group dash events by entity to avoid multiple dash events for the same entity
    let mut dash_map = HashMap::new();
    for event in events.read() {
        dash_map.insert(event.entity, event.direction);
    }

    // Process dash events for each entity
    for (entity, direction) in dash_map {
        if let Ok((mut dash, mut boost, mut movement_stat, mut velocity)) =
            q_spaceships.get_mut(entity)
        {
            // Check if the spaceship can dash
            if boost.energy >= dash.energy_cost && dash.current_cooldown <= 0.0 {
                // Start the dash
                dash.is_dashing = true;
                dash.direction = direction;
                dash.duration = DASH_DURATION;
                boost.energy -= dash.energy_cost;

                // Set dash velocity
                velocity.0 = dash.direction * dash.speed;

                // Reduce linear damping for sharp movement during the dash
                movement_stat.towards_linear_damping(0.0, damp_factor);
            }
        }
    }
}

fn handle_boost(
    time: Res<Time>,
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_spaceships: Query<(&mut Boost, &Spaceship)>,
    player_infos: Res<PlayerInfos>,
) {
    let delta = time.delta_seconds();

    for (action, id) in q_actions.iter() {
        let Some(&entity) = player_infos[PlayerInfoType::Spaceship].get(id) else {
            continue;
        };

        let Ok((mut boost, _spaceship)) = q_spaceships.get_mut(entity) else {
            continue;
        };

        let is_boosting = action.pressed(&PlayerAction::Boost);
        boost.is_boosting = is_boosting && boost.can_boost(delta);
        boost.update_state(delta, is_boosting);
    }
}

fn apply_movement(
    time: Res<Time>,
    mut query: Query<(
        &DesiredVelocity,
        &mut LinearVelocity,
        &MovementStat,
        &mut Dash,
        &Spaceship,
        Option<&Boost>,
    )>,
) {
    let delta = time.delta_seconds();
    let delta_velocity = delta * 1.2;

    query.par_iter_mut().for_each(
        |(desired_velocity, mut linear, _movement_stat, mut dash, spaceship, boost)| {
            // Handle dash state
            if dash.is_dashing {
                dash.duration -= delta;
                if dash.duration <= 0.0 {
                    dash.is_dashing = false;
                    dash.current_cooldown = dash.cooldown;
                }
            } else if dash.current_cooldown > 0.0 {
                dash.current_cooldown -= delta;
            }

            // Apply movement if not dashing
            if !dash.is_dashing {
                let max_speed = match boost {
                    Some(boost) if boost.is_boosting => spaceship.max_linear_speed * 1.5,
                    _ => spaceship.max_linear_speed,
                };

                linear.0 += desired_velocity.0 * delta_velocity;
                linear.0 = linear.0.clamp_length_max(max_speed);
            }
        },
    );
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
    // Whether the boost is actively being used.
    pub is_boosting: bool,
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
