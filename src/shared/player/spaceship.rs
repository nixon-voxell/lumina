use avian2d::prelude::*;
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use blenvy::BlueprintInfo;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::action::PlayerAction;
use crate::shared::physics::PhysicsBundle;
use crate::shared::SourceEntity;

use super::{BlueprintType, PlayerId, PlayerInfoType, PlayerInfos};

pub(super) struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spaceship_movement);

        app.register_type::<SpaceShipType>()
            .register_type::<SpaceShip>();
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
    mut q_spaceships: Query<(
        &mut MovementStat,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut LinearDamping,
        &Rotation,
        &SpaceShip,
    )>,
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
        let Some(&spaceship_entity) = player_infos[PlayerInfoType::SpaceShip].get(id) else {
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

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub enum SpaceShipType {
    Assassin,
    Tank,
    Support,
}

impl BlueprintType for SpaceShipType {
    fn visual_info(&self) -> BlueprintInfo {
        match self {
            SpaceShipType::Assassin => {
                BlueprintInfo::from_path("levels/SpaceShipAssassinVisual.glb")
            }
            _ => todo!("{self:?} is not supported yet."),
        }
    }

    fn config_info(&self) -> BlueprintInfo {
        match self {
            SpaceShipType::Assassin => {
                BlueprintInfo::from_path("levels/SpaceShipAssassinConfig.glb")
            }
            _ => todo!("{self:?} is not supported yet."),
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
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

impl Component for SpaceShip {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let entity_ref = world.entity(entity);

            // Do not initialize for spaceships that are spawned remotely.
            if entity_ref.contains::<Replicated>() {
                return;
            }

            let spaceship = entity_ref.get::<Self>().unwrap();
            // TODO: Consider using a lookup collider.
            let collider = Collider::triangle(
                Vec2::new(-20.0, 20.0),
                Vec2::new(-20.0, -20.0),
                Vec2::new(20.0, 0.0),
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

            info!("\n\n Initialized spaceship for {entity:?}");
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