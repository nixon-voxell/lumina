use std::ops::{Index, IndexMut};

use avian2d::prelude::*;
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use lumina_common::prelude::*;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use super::PlayerId;

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AmmoRefEntities>()
            .init_resource::<AmmoPools>()
            .add_event::<FireAmmo>()
            .add_systems(Startup, spawn_ammo_ref)
            .add_systems(Update, setup_ammmo_ref)
            .add_systems(FixedUpdate, (fire_ammo, track_ammo_lifetime));
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
    mut ammo_refs: ResMut<AmmoRefEntities>,
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
    mut ammo_pools: ResMut<AmmoPools>,
    ammo_refs: Res<AmmoRefEntities>,
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

        let ammo_entity = match ammo_pool.get_unused() {
            Some(ammo_entity) => ammo_entity,
            None => {
                let ammo_entity = commands
                    .spawn(FireAmmoBundle {
                        ammo_type: fire_ammo.ammo_type,
                        rigidbody: RigidBody::Dynamic,
                        sensor: Sensor,
                        mass_properties: MassPropertiesBundle::new_computed(collider, 1.0),
                        collider: collider.clone(),
                        spatial: SpatialBundle::default(),
                        source: SourceEntity,
                    })
                    .id();
                ammo_pool.used.insert(ammo_entity);

                ammo_entity
            }
        };

        let rotation = fire_ammo.direction.to_angle();
        // Initialize the ammo position and rotation.
        commands
            .entity(ammo_entity)
            // Enable ammo
            .remove::<AmmoDisabled>()
            .insert((
                AmmoStat {
                    lifetime: ammo.lifetime,
                },
                Position(fire_ammo.position),
                Rotation::radians(rotation),
                LinearVelocity(fire_ammo.direction * ammo.linear_impulse),
                AngularVelocity(ammo.angular_impulse),
                AmmoDamage(fire_ammo.damage),
                Transform::from_xyz(fire_ammo.position.x, fire_ammo.position.y, -10.0)
                    .with_rotation(Quat::from_rotation_z(rotation)),
            ));
    }
}

fn track_ammo_lifetime(
    mut commands: Commands,
    mut q_ammos: Query<(&mut AmmoStat, &AmmoType, Entity), Without<AmmoDisabled>>,
    mut ammo_pools: ResMut<AmmoPools>,
    time: Res<Time>,
) {
    for (mut ammo, ammo_type, ammo_entity) in q_ammos.iter_mut() {
        ammo.lifetime -= time.delta_seconds();

        if ammo.lifetime <= 0.0 {
            commands.entity(ammo_entity).insert(AmmoDisabled);
            ammo_pools[*ammo_type].set_unused(ammo_entity);
        }
    }
}

#[derive(Event)]
pub struct FireAmmo {
    #[allow(unused)]
    pub id: PlayerId,
    pub ammo_type: AmmoType,
    pub position: Vec2,
    pub direction: Vec2,
    pub damage: f32,
}

#[derive(Bundle)]
pub struct FireAmmoBundle {
    pub ammo_type: AmmoType,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub mass_properties: MassPropertiesBundle,
    pub collider: Collider,
    pub spatial: SpatialBundle,
    pub source: SourceEntity,
}

#[derive(Component)]
pub struct AmmoStat {
    /// Duration left before the ammo expires.
    pub lifetime: f32,
}

#[allow(unused)]
/// Ammo damage applied to damagable objects.
#[derive(Component)]
pub struct AmmoDamage(pub f32);

/// Tag component to notify that an ammo has been disabled and therefore rendered useless.
pub struct AmmoDisabled;

impl Component for AmmoDisabled {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _| {
            world
                .commands()
                .entity(entity)
                // Hide ammo.
                .insert(Visibility::Hidden)
                // Stop any physics acting on the ammo.
                .remove::<RigidBody>();
        });

        hooks.on_remove(|mut world, entity, _| {
            // Entity might have been despawned...
            if let Some(mut entity_cmds) = world.commands().get_entity(entity) {
                entity_cmds.insert((
                    // Show ammo.
                    Visibility::Inherited,
                    // Enable physics.
                    RigidBody::Dynamic,
                ));
            }
        });
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct AmmoPools<const COUNT: usize = { AmmoType::COUNT }>([AmmoPool; COUNT]);

impl<const COUNT: usize> Index<AmmoType> for AmmoPools<COUNT> {
    type Output = AmmoPool;

    fn index(&self, index: AmmoType) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<const COUNT: usize> IndexMut<AmmoType> for AmmoPools<COUNT> {
    fn index_mut(&mut self, index: AmmoType) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<const COUNT: usize> Default for AmmoPools<COUNT> {
    fn default() -> Self {
        Self(std::array::from_fn(|_| AmmoPool::default()))
    }
}

#[derive(Default)]
pub struct AmmoPool {
    pub used: EntityHashSet,
    pub unused: EntityHashSet,
}

impl AmmoPool {
    /// Get an unused ammo from the ammo pool and move it from the [`Self::unused`] to [`Self::used`].
    pub fn get_unused(&mut self) -> Option<Entity> {
        let entity = self.unused.iter().next().copied()?;
        self.unused.remove(&entity);
        self.used.insert(entity);

        Some(entity)
    }

    /// Set ammo as unused (normally used when ammo becomes irrelevant).
    ///
    /// # Returns
    ///
    /// True if successful, false if unsuccessful.
    pub fn set_unused(&mut self, entity: Entity) {
        if self.used.remove(&entity) == false {
            error!("Ammo entity was not from the ammo pool!");
        }

        self.unused.insert(entity);
    }
}

/// Entities with [`AmmoRef`] component and the associated [`Ammo`] reference data.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct AmmoRefEntities(HashMap<AmmoType, Entity>);

/// Point of reference for a certain [`AmmoType`].
/// Use this when creating an ammo prefab in Blender.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AmmoRef;

#[derive(Component, Reflect, EnumCount, EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub enum AmmoType {
    LongRange,
    // ShortRange,
    // Honing,
}

impl BlueprintType for AmmoType {
    fn visual_info(&self) -> BlueprintInfo {
        match self {
            AmmoType::LongRange => BlueprintInfo::from_path("levels/AmmoLongRangeVisual.glb"),
            // _ => todo!("{self:?} is not supported yet."),
        }
    }

    fn config_info(&self) -> BlueprintInfo {
        match self {
            AmmoType::LongRange => BlueprintInfo::from_path("levels/AmmoLongRangeConfig.glb"),
            // _ => todo!("{self:?} is not supported yet."),
        }
    }
}

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
