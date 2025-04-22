use std::hash::Hash;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::view::{NoFrustumCulling, VisibilitySystems};
use bevy::utils::HashMap;
use bevy_enoki::prelude::*;
use bevy_enoki::EnokiPlugin;
use lumina_common::prelude::*;
use smallvec::SmallVec;
use strum::{AsRefStr, EnumCount, EnumIter, IntoEnumIterator};

pub(super) struct ParticleVfxPlugin;

impl Plugin for ParticleVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnokiPlugin)
            .add_plugins(particle_from_component_plugin::<MuzzleFlashMaterial>)
            .add_systems(
                PreStartup,
                (
                    load_particle_effects::<DespawnVfxType>,
                    load_particle_effects::<InPlaceVfxType>,
                ),
            )
            .add_systems(Update, setup_in_place_vfx)
            .add_systems(
                PostUpdate,
                do_not_cull_vfx.before(VisibilitySystems::CheckVisibility),
            )
            .observe(one_shot_vfx::<ColorParticle2dMaterial>)
            .observe(one_shot_vfx::<AmmoHitMaterial>);
    }
}

// A hack for making all vfx still visible even if some of them are off screen.
fn do_not_cull_vfx(
    mut commands: Commands,
    mut q_vfx: Query<Entity, Added<ParticleEffectInstance>>,
) {
    for entity in q_vfx.iter_mut() {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}

#[derive(Component, Reflect, Asset, AsBindGroup, Default, Debug, Clone)]
#[reflect(Component)]
pub struct MuzzleFlashMaterial {}

impl Particle2dMaterial for MuzzleFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enoki/muzzle_flash.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Default, Clone, Copy)]
pub struct AmmoHitMaterial {}

impl Particle2dMaterial for AmmoHitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enoki/ammo_hit.wgsl".into()
    }
}

fn setup_in_place_vfx(
    mut commands: Commands,
    q_vfxs: Query<(&InPlaceVfxType, &Transform, Entity), Added<InPlaceVfxType>>,
    effects: Res<InPlaceVfxAssets>,
) {
    for (&vfx, &transform, entity) in q_vfxs.iter() {
        commands.entity(entity).insert(ParticleSpawnerBundle {
            state: ParticleSpawnerState {
                active: false,
                ..default()
            },
            effect: effects[vfx as usize].clone_weak(),
            material: DEFAULT_MATERIAL,
            transform,
            ..default()
        });
    }
}

fn one_shot_vfx<M: Particle2dMaterial + Default>(
    trigger: Trigger<DespawnVfx<M>>,
    mut commands: Commands,
    assets: Res<DespawnVfxEffects>,
) {
    let vfx = trigger.event();
    commands.spawn((
        ParticleSpawnerBundle {
            effect: assets[vfx.index()].clone_weak(),
            material: vfx.material.clone_weak(),
            transform: vfx.transform,
            ..default()
        },
        OneShot::Despawn,
    ));
}

fn load_particle_effects<T: EnumCount + IntoEnumIterator + AsRef<str> + Send + Sync + 'static>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut assets = EnumVariantRes::<T, Handle<Particle2dEffect>>::default();

    for (i, vfx) in T::iter().enumerate() {
        assets[i] = asset_server.load(vfx.as_ref().to_string() + ".ron");
    }

    commands.insert_resource(assets);
}

/// Initialize and update [`InPlaceVfxMap`].
pub struct InPlaceVfxMapPlugin<T: Component>(PhantomData<T>);

impl<T: Component> Plugin for InPlaceVfxMapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_in_place_vfx_map::<T>, map_in_place_vfx::<T>).chain(),
        );
    }
}

impl<T: Component> Default for InPlaceVfxMapPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Insert [`InPlaceVfxMap`] component to target entities that just spawned.
fn setup_in_place_vfx_map<T: Component>(
    mut commands: Commands,
    q_targets: Query<
        Entity,
        (
            With<T>,
            With<SourceEntity>,
            Or<(Added<T>, Added<SourceEntity>)>,
            Without<InPlaceVfxMap>,
        ),
    >,
) {
    for entity in q_targets.iter() {
        commands.entity(entity).insert(InPlaceVfxMap::default());
    }
}

/// Find parent with component `T` when any vfx is spawned and setup [`InPlaceVfxMap`].
fn map_in_place_vfx<T: Component>(
    mut commands: Commands,
    mut q_targets: Query<&mut InPlaceVfxMap, With<T>>,
    q_vfxs: Query<(&InPlaceVfxType, Entity), (Without<MappedInPlaceVfx>, With<SourceEntity>)>,
    q_parents: Query<&Parent>,
) {
    for (vfx, vfx_entity) in q_vfxs.iter() {
        for map_entity in q_parents.iter_ancestors(vfx_entity) {
            let Ok(mut map) = q_targets.get_mut(map_entity) else {
                continue;
            };

            debug!("Mapped: {vfx:?} from {vfx_entity} to {map_entity}");
            if let Some(entities) = map.get_mut(vfx) {
                entities.push(vfx_entity);
            } else {
                map.insert(*vfx, SmallVec::from_elem(vfx_entity, 1));
            }

            commands.entity(vfx_entity).insert(MappedInPlaceVfx);

            // Only find the first parent with the required component.
            break;
        }
    }
}

/// Mappings to vfx entities that is part of the children hierarchy of the current entity.
#[derive(Component, Deref, DerefMut, Default)]
pub struct InPlaceVfxMap(HashMap<InPlaceVfxType, SmallVec<[Entity; 1]>>);

pub type InPlaceVfxAssets = EnumVariantRes<InPlaceVfxType, Handle<Particle2dEffect>>;

#[derive(
    Event, Reflect, AsRefStr, EnumIter, EnumCount, Debug, Clone, Copy, Hash, PartialEq, Eq,
)]
#[reflect(Component)]
#[strum(prefix = "enoki/", serialize_all = "snake_case")]
pub enum InPlaceVfxType {
    GunSparks,
    MuzzleFlash,
    BoosterFlakes,
    Smoke,
    Lumina,
}

/// Marker for [`InPlaceVfxType`] that has already been mapped.
#[derive(Component)]
pub struct MappedInPlaceVfx;

pub type DespawnVfxEffects = EnumVariantRes<DespawnVfxType, Handle<Particle2dEffect>>;

#[derive(AsRefStr, EnumIter, EnumCount, Debug, Clone, Copy)]
#[strum(prefix = "enoki/", serialize_all = "snake_case")]
pub enum DespawnVfxType {
    AmmoHit,
}

/// Triggers a [`DespawnVfxType`] that will despawn after the vfx completes.
///
/// # Usage
///
/// ```
/// use bevy::prelude::*;
/// use bevy_enoki::prelude::*;
/// use lumina_vfx::prelude::*;
///
/// fn spawn_vfx(mut commands: Commands) {
///     commands.trigger(DespawnVfx {
///         vfx_type: DespawnVfxType::AmmoHit,
///         transform: Transform::default(),
///         material: DEFAULT_MATERIAL,
///     });
/// }
/// ```
#[derive(Event)]
pub struct DespawnVfx<M: Particle2dMaterial> {
    pub vfx_type: DespawnVfxType,
    pub transform: Transform,
    pub material: Handle<M>,
}

impl<M: Particle2dMaterial> DespawnVfx<M> {
    pub fn index(&self) -> usize {
        self.vfx_type as usize
    }
}

pub fn particle_from_component_plugin<M: AssetFromComponent + Particle2dMaterial>(app: &mut App)
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    app.add_plugins((
        AssetFromComponentPlugin::<M>::default(),
        Particle2dMaterialPlugin::<M>::default(),
    ))
    .add_systems(Update, remove_color_material::<M>);
}

fn remove_color_material<M: AssetFromComponent>(
    mut commands: Commands,
    q_entities: Query<Entity, (With<M>, Added<Handle<ColorParticle2dMaterial>>)>,
) {
    for entity in q_entities.iter() {
        commands
            .entity(entity)
            .remove::<Handle<ColorParticle2dMaterial>>();
    }
}
