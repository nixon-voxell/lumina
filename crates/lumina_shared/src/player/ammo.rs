use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_transform_interpolation::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use strum::IntoEnumIterator;

use crate::blueprints::AmmoType;
use crate::health::Health;

use super::{prelude::TeamType, GameLayer, PlayerId};

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RefEntityMap<AmmoType>>()
            .init_resource::<EntityPools<AmmoType>>()
            .add_systems(Startup, spawn_ammo_ref)
            .add_systems(Update, setup_ammmo_ref)
            .add_systems(FixedUpdate, (ammo_collision, track_ammo_lifetime).chain())
            .observe(fire_ammo);
    }
}

/// Initialize ammo references from Blender.
/// Each reference should have the [`AmmoRef`] component.
fn spawn_ammo_ref(mut commands: Commands) {
    for ammo_type in AmmoType::iter() {
        commands.spawn((ammo_type.config_info(), SpawnBlueprint));
    }
}

fn setup_ammmo_ref(
    q_ammo_refs: Query<(&AmmoType, Entity), Added<AmmoRef>>,
    mut ammo_refs: ResMut<RefEntityMap<AmmoType>>,
) {
    for (ammo_type, entity) in q_ammo_refs.iter() {
        debug!("Initialized ammo ref {:?}.", ammo_type);
        ammo_refs.insert(*ammo_type, entity);
    }
}

fn fire_ammo(
    trigger: Trigger<FireAmmo>,
    mut commands: Commands,
    q_weapons: Query<(&AmmoStat, &PlayerId, &TeamType, &WorldIdx)>,
    q_ammo_refs: Query<&Collider, With<AmmoRef>>,
    mut ammo_pools: ResMut<EntityPools<AmmoType>>,
    ammo_refs: Res<RefEntityMap<AmmoType>>,
) {
    let &FireAmmo {
        weapon_entity,
        position,
        direction,
    } = trigger.event();

    let Ok((
        AmmoStat {
            lifetime,
            fire,
            ammo_type,
            ..
        },
        &player_id,
        &team_type,
        &world_id,
    )) = q_weapons.get(weapon_entity)
    else {
        return;
    };

    let Some(collider) = ammo_refs
        .get(ammo_type)
        .and_then(|e| q_ammo_refs.get(*e).ok())
    else {
        return;
    };

    // Get ammo pool for the particular ammo type.
    let ammo_pool = &mut ammo_pools[*ammo_type as usize];

    let ammo_entity = ammo_pool.get_unused_or_spawn(|| {
        commands
            .spawn(InitAmmoBundle::new(*ammo_type, collider.clone()))
            .insert(Name::new(ammo_type.as_ref().to_string()))
            .id()
    });

    // Initialize fire ammo components.
    commands.entity(ammo_entity).insert(FireAmmoBundle {
        player_id,
        team_type,
        world_id,
        position: position.into(),
        rotation: Rotation::radians(direction.to_angle()),
        linear_velocity: LinearVelocity(direction * fire.linear_impulse),
        angular_velocity: fire.angular_impulse.into(),
        linear_damping: fire.linear_damping.into(),
        angular_damping: fire.angular_damping.into(),
        lifetime: AmmoLifetime(Timer::from_seconds(*lifetime, TimerMode::Once)),
        weapon_ref: AmmoWeaponRef(weapon_entity),
        visibility: Visibility::Inherited,
        rigidbody: RigidBody::Dynamic,
    });
}

/// Track ammo lifetime and set unused once it reaches its lifetime.
fn track_ammo_lifetime(
    mut commands: Commands,
    mut q_ammos: Query<
        (
            &mut AmmoLifetime,
            &AmmoType,
            &mut Visibility,
            &mut CollidingEntities,
            Entity,
        ),
        With<RigidBody>,
    >,
    mut ammo_pools: ResMut<EntityPools<AmmoType>>,
    time: Res<Time>,
) {
    for (mut lifetime, ammo_type, mut viz, mut colliding, entity) in q_ammos.iter_mut() {
        if lifetime.finished() {
            // Hide and clear collisions.
            *viz = Visibility::Hidden;
            colliding.clear();
            commands.entity(entity).remove::<RigidBody>();
            ammo_pools[*ammo_type as usize].set_unused(entity);
        }

        lifetime.tick(time.delta());
    }
}

fn ammo_collision(
    mut commands: Commands,
    q_col_criteria: Query<(Option<&PlayerId>, Has<Sensor>)>,
    // Only apply damage on the server.
    mut q_healths: Query<(&mut Health, Option<&TeamType>), With<server::SyncTarget>>,
    q_ammo_stats: Query<&AmmoStat, With<SourceEntity>>,
    mut q_ammos: Query<
        (
            &Position,
            &Rotation,
            &AmmoWeaponRef,
            &mut AmmoLifetime,
            Ref<CollidingEntities>,
            &Visibility,
            &PlayerId,
            &TeamType,
        ),
        (
            Changed<CollidingEntities>,
            Without<AmmoRef>,
            With<RigidBody>,
        ),
    >,
    q_rigidbodies: Query<&RigidBody>,
    // network_identity: NetworkIdentity,
) {
    for (position, rotation, weapon_ref, mut lifetime, colliding, viz, id, team_type) in
        q_ammos.iter_mut()
    {
        // Skip already hidden ammos.
        if viz == Visibility::Hidden {
            continue;
        }
        // No collisions.
        if colliding.is_added() || colliding.is_empty() {
            continue;
        }

        let Ok(AmmoStat {
            knockback, effect, ..
        }) = q_ammo_stats.get(weapon_ref.0)
        else {
            continue;
        };

        let mut hit = false;

        for &entity in colliding.iter() {
            // Ignore if we are colliding with entity that
            // has similar player id or has Sensor component.
            if q_col_criteria
                .get(entity)
                .is_ok_and(|(col_id, has_sensor)| {
                    col_id.is_some_and(|col_id| col_id == id) || has_sensor
                })
            {
                continue;
            }

            // Apply damage if not in the same team or there is no team.
            if let Ok((mut health, col_team_type)) = q_healths.get_mut(entity) {
                if col_team_type.is_some_and(|t| t != team_type) || col_team_type.is_none() {
                    **health -= effect.damage;
                }
            }

            // Apply force if possible.
            if q_rigidbodies
                .get(entity)
                .is_ok_and(|rigidbody| rigidbody == &RigidBody::Dynamic)
            {
                let mut impulse = ExternalImpulse::default();
                let impulse_direction = rotation * Vec2::X;

                impulse.apply_impulse_at_point(
                    impulse_direction * knockback.angular_impulse,
                    position.0,
                    Vec2::ZERO,
                );
                impulse.set_impulse(impulse_direction * knockback.impulse);
                commands.entity(entity).insert(impulse);
            }

            hit = true;
        }

        // Ammo collided with something, disable it!
        if hit {
            let duration = lifetime.duration();
            lifetime.tick(duration);
            commands.trigger(AmmoHit(*position));
        }
    }
}

#[derive(Bundle)]
pub struct InitAmmoBundle {
    pub ammo_type: AmmoType,
    pub mass_properties: MassPropertiesBundle,
    pub collider: Collider,
    pub layers: CollisionLayers,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub spatial: SpatialBundle,
    pub no_translation_interp: NoTranslationInterpolation,
    pub source: SourceEntity,
}

impl InitAmmoBundle {
    pub fn new(ammo_type: AmmoType, collider: Collider) -> Self {
        Self {
            ammo_type,
            mass_properties: MassPropertiesBundle::new_computed(&collider, 1.0),
            collider,
            layers: CollisionLayers::new(GameLayer::Ammo, [GameLayer::Spaceship]),
            rigidbody: RigidBody::Dynamic,
            sensor: Sensor,
            spatial: SpatialBundle {
                // Set z axis so that it renders behind walls and spaceships.
                transform: Transform::from_xyz(0.0, 0.0, -10.0),
                ..default()
            },
            no_translation_interp: NoTranslationInterpolation,
            source: SourceEntity,
        }
    }
}

#[derive(Bundle)]
pub struct FireAmmoBundle {
    pub player_id: PlayerId,
    pub team_type: TeamType,
    pub world_id: WorldIdx,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub lifetime: AmmoLifetime,
    pub weapon_ref: AmmoWeaponRef,
    pub visibility: Visibility,
    pub rigidbody: RigidBody,
}

/// Stores the weapon that fires the ammo.
#[derive(Event)]
pub struct FireAmmo {
    pub weapon_entity: Entity,
    pub position: Vec2,
    pub direction: Vec2,
}

#[derive(Event, Deref)]
pub struct AmmoHit(Position);

/// Reference to the weapon entity that fired the ammo.
#[derive(Component)]
pub struct AmmoWeaponRef(Entity);

/// Point of reference for a certain [`AmmoType`].
/// Use this when creating an ammo prefab in Blender.
///
/// This is usually used to store ammo specific settings,
/// (e.g. the collider of the ammo.)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AmmoRef;

/// The current lifetime of the ammo,
/// ammo will cease to exist when the timer finishes.
#[derive(Component, Deref, DerefMut)]
pub struct AmmoLifetime(Timer);

/// Attached to the [`super::Weapon`] entity to store the stat of the weapon.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AmmoStat {
    /// The duration the ammo lives.
    lifetime: f32,
    /// Applied when ammo is fired.
    fire: AmmoFire,
    /// Type of ammo (for visual only).
    ammo_type: AmmoType,
    /// The effect to apply on the collided entity.
    effect: AmmoEffect,
    /// Knockback effect when the ammo applies its effect.
    knockback: AmmoKnockback,
}

/// Ammo effect applied to when it hits a [`Collider`].
#[derive(Reflect)]
pub struct AmmoEffect {
    damage: f32,
    /// Optional AOE.
    radius: Option<f32>,
    /// Does it bounces off colliders until it found
    /// its target or completes its lifetime?
    bounce: bool,
}

/// Initial fire ammo physics.
#[derive(Reflect)]
pub struct AmmoFire {
    /// Initial impulse linear velocity when the ammo is fired.
    linear_impulse: f32,
    /// Initial impulse angular velocity when the ammo is fired.
    angular_impulse: f32,
    /// Linear damping value for the ammo.
    linear_damping: f32,
    /// Angular damping value for the ammo.
    angular_damping: f32,
}

/// Knockback effect of an ammo when it hits the target.
#[derive(Reflect)]
pub struct AmmoKnockback {
    /// Knockback impulse,
    /// used for [`ExternalImpulse::apply_impulse`].
    impulse: f32,
    /// Knockback angular impulse,
    /// used for [`ExternalImpulse::apply_impulse_at_point`].
    angular_impulse: f32,
}
