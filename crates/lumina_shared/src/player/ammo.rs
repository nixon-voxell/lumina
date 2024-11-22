use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_transform_interpolation::*;
use blenvy::*;
use lumina_common::prelude::*;
use strum::IntoEnumIterator;

use crate::blueprints::AmmoType;
use crate::health::Health;

use super::PlayerId;

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RefEntityMap<AmmoType>>()
            .init_resource::<EntityPools<AmmoType>>()
            .add_event::<FireAmmo>()
            .add_systems(Startup, spawn_ammo_ref)
            .add_systems(Update, setup_ammmo_ref)
            .add_systems(FixedUpdate, (fire_ammo, apply_damage))
            .add_systems(FixedPostUpdate, track_ammo_lifetime);
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
    for (ammo_type, ammo_entity) in q_ammo_refs.iter() {
        debug!("Initialized ammo ref {:?}.", ammo_type);
        ammo_refs.insert(*ammo_type, ammo_entity);
    }
}

fn fire_ammo(
    mut commands: Commands,
    q_ammo_refs: Query<(&Ammo, &Collider), With<AmmoRef>>,
    mut fire_ammo_evr: EventReader<FireAmmo>,
    mut ammo_pools: ResMut<EntityPools<AmmoType>>,
    ammo_refs: Res<RefEntityMap<AmmoType>>,
) {
    for fire_ammo in fire_ammo_evr.read() {
        let Some((ammo, collider)) = ammo_refs
            .get(&fire_ammo.ammo_type)
            .and_then(|e| q_ammo_refs.get(*e).ok())
        else {
            continue;
        };

        // Get ammo pool for the particular ammo type.
        let ammo_pool = &mut ammo_pools[fire_ammo.ammo_type];

        let ammo_entity = ammo_pool.get_unused_or_spawn(|| {
            commands
                .spawn(InitAmmoBundle::new(fire_ammo.ammo_type, collider.clone()))
                .id()
        });

        // Initialize fire ammo components.
        commands
            .entity(ammo_entity)
            .insert(FireAmmoBundle::new(fire_ammo, ammo));
    }
}

fn track_ammo_lifetime(
    mut commands: Commands,
    mut q_ammos: Query<(&mut AmmoStat, &AmmoType, Entity), With<RigidBody>>,
    mut ammo_pools: ResMut<EntityPools<AmmoType>>,
    time: Res<Time>,
) {
    for (mut ammo_stat, ammo_type, ammo_entity) in q_ammos.iter_mut() {
        ammo_stat.lifetime -= time.delta_seconds();

        if ammo_stat.lifetime <= 0.0 {
            commands
                .entity(ammo_entity)
                .insert(Visibility::Hidden)
                .remove::<RigidBody>();
            ammo_pools[*ammo_type].set_unused(ammo_entity);
        }
    }
}

// TODO: Prevent ammos colliding with itself.
/// Applies damage to the targeted spaceship entities based on received DamageEvents.
fn apply_damage(
    mut q_healths: Query<&mut Health>,
    mut q_ammos: Query<
        (&mut AmmoStat, &AmmoDamage, &CollidingEntities),
        Changed<CollidingEntities>,
    >,
) {
    for (mut stat, &AmmoDamage(damage), colliding) in q_ammos.iter_mut() {
        // Ammo collided with something, disable it!
        stat.lifetime = 0.0;

        for entity in colliding.iter() {
            if let Ok(mut health) = q_healths.get_mut(*entity) {
                **health -= damage;
            }
        }
    }
}

#[derive(Event)]
pub struct FireAmmo {
    pub player_id: PlayerId,
    pub world_id: PhysicsWorldId,
    pub ammo_type: AmmoType,
    pub position: Vec2,
    pub direction: Vec2,
    pub damage: f32,
}

#[derive(Bundle)]
pub struct InitAmmoBundle {
    pub ammo_type: AmmoType,
    pub mass_properties: MassPropertiesBundle,
    pub collider: Collider,
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
            rigidbody: RigidBody::Dynamic,
            sensor: Sensor,
            spatial: SpatialBundle::default(),
            no_translation_interp: NoTranslationInterpolation,
            source: SourceEntity,
        }
    }
}

#[derive(Bundle)]
pub struct FireAmmoBundle {
    pub player_id: PlayerId,
    pub world_id: PhysicsWorldId,
    pub stat: AmmoStat,
    pub damage: AmmoDamage,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub rigidbody: RigidBody,
    pub visibility: Visibility,
}

impl FireAmmoBundle {
    pub fn new(fire_ammo: &FireAmmo, ammo: &Ammo) -> Self {
        Self {
            player_id: fire_ammo.player_id,
            world_id: fire_ammo.world_id,
            stat: AmmoStat {
                lifetime: ammo.lifetime,
            },
            damage: AmmoDamage(fire_ammo.damage),
            position: Position(fire_ammo.position),
            rotation: Rotation::radians(fire_ammo.direction.to_angle()),
            linear_velocity: LinearVelocity(fire_ammo.direction * ammo.linear_impulse),
            angular_velocity: AngularVelocity(ammo.angular_impulse),
            rigidbody: RigidBody::Dynamic,
            visibility: Visibility::Inherited,
        }
    }
}

#[derive(Component)]
pub struct AmmoStat {
    /// Duration left before the ammo expires.
    pub lifetime: f32,
}

/// Ammo damage applied to damagable objects.
#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct AmmoDamage(pub f32);

/// Point of reference for a certain [`AmmoType`].
/// Use this when creating an ammo prefab in Blender.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AmmoRef;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Ammo {
    /// Duration the ammo stays relevant.
    lifetime: f32,
    /// Initial impulse linear velocity when the ammo is fired.
    linear_impulse: f32,
    /// Initial impulse angular velocity when the ammo is fired.
    angular_impulse: f32,
    /// Linear damping value for the ammo.
    linear_damping: f32,
    /// Angular damping value for the ammo.
    angular_damping: f32,
}
