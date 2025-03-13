use avian2d::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use lightyear::prelude::*;
use lumina_common::prelude::*;
use server::*;
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr};

use super::prelude::*;

pub(super) struct SpawnPointPlugin;

impl Plugin for SpawnPointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnPointParentCache>()
            .add_systems(Update, init_spawn_point_to_parent)
            .add_systems(Update, cache_spawn_point)
            .add_systems(
                PostUpdate,
                init_spaceships_at_spawn_points.after(TransformSystem::TransformPropagate),
            )
            .observe(on_add_spawned)
            .observe(on_remove_spawned)
            .observe(on_spawn_point_freed);
    }
}

/// Cache the [`SpawnPoint`]s.
fn cache_spawn_point(
    q_spawn_points: Query<(&WorldIdx, Entity), (With<SpawnPointParent>, Added<WorldIdx>)>,
    mut cache: ResMut<SpawnPointParentCache>,
) {
    for (world_idx, entity) in q_spawn_points.iter() {
        cache.insert(world_idx.room_id(), entity);
    }
}

/// Initialize spaceships and assign them to available spawn points.
fn init_spaceships_at_spawn_points(
    mut commands: Commands,
    mut q_spawn_parents: Query<&mut SpawnPointParent>,
    q_spawn_points: Query<(&GlobalTransform, &SpawnPoint, Entity), Without<SpawnPointUsed>>,
    mut q_spaceship: Query<
        (
            &mut Position,
            &mut Rotation,
            &PlayerId,
            Option<&TeamType>,
            Entity,
            &WorldIdx,
        ),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<SpawnPointEntity>,
        ),
    >,
    network_identity: NetworkIdentity,
    cache: Res<SpawnPointParentCache>,
) {
    for (mut position, mut rotation, id, team_type, spaceship_entity, world_id) in
        q_spaceship.iter_mut()
    {
        let Some(mut spawn_parent) = cache
            .get(&world_id.room_id())
            .and_then(|&e| q_spawn_parents.get_mut(e).ok())
        else {
            continue;
        };

        // If we are in multiplayer mode, let the server handle the spawn position.
        if id.is_local() == false && network_identity.is_client() {
            return;
        }

        let spawn_point = match team_type {
            // Get the desired spawn point based on the appointed team type.
            Some(team_type) => spawn_parent[*team_type as usize].get_unused(),
            // Get a spawn point that has the least player in it.
            None => {
                let team_a_size = spawn_parent[TeamType::A as usize].used().len();
                let team_b_size = spawn_parent[TeamType::B as usize].used().len();

                match team_a_size > team_b_size {
                    true => spawn_parent[TeamType::B as usize].get_unused(),
                    false => spawn_parent[TeamType::A as usize].get_unused(),
                }
            }
        };

        let Some((spawn_transform, &SpawnPoint(team_type), spawn_point_entity)) =
            spawn_point.and_then(|e| q_spawn_points.get(e).ok())
        else {
            error!("Unable to find spawn point for {spaceship_entity}!");
            return;
        };

        // Extract position and rotation from the spawn point's transform
        let (_, spawn_rotation, spawn_translation) =
            spawn_transform.to_scale_rotation_translation();

        // Set spaceship position and rotation based on the spawn point's transform
        *position = Position(spawn_translation.xy());
        *rotation = Rotation::radians(spawn_rotation.to_scaled_axis().z);

        // Associate the spaceship with the spawn point, mark it as used, and assign its team type
        commands
            .entity(spaceship_entity)
            .insert((SpawnPointEntity(spawn_point_entity), team_type));
    }
}

fn init_spawn_point_to_parent(
    q_spawns: Query<(&SpawnPoint, &Parent, Entity), Added<SpawnPoint>>,
    mut q_spawn_parents: Query<&mut SpawnPointParent>,
) {
    for (&SpawnPoint(team_type), parent, entity) in q_spawns.iter() {
        let Ok(mut spawn_parent) = q_spawn_parents.get_mut(parent.get()) else {
            error!("SpawnPoint ({entity}) spawned without SpawnPointParent!");
            return;
        };

        if spawn_parent[team_type as usize].insert_new_unused(entity) == false {
            warn!("Same SpawnPoint ({entity}) added twice!")
        }
        info!("Initialized SpawnPoint ({entity})");
    }
}

/// When a [`SpawnPointEntity`] is being added,
/// consume it so that it can't be used again.
fn on_add_spawned(
    trigger: Trigger<OnAdd, SpawnPointEntity>,
    mut commands: Commands,
    query: Query<&SpawnPointEntity>,
) {
    let spaceship_entity = trigger.entity();
    let spawn_point_entity = **query.get(trigger.entity()).unwrap();

    commands
        .entity(spawn_point_entity)
        .insert(SpawnPointUsed(spaceship_entity));
}

// FIXME: This will cause a panic!
// Because the component will be removed first before the trigger event!
/// When a [`SpawnPointEntity`] is being removed,
/// release it so that it can be reused again.
fn on_remove_spawned(
    trigger: Trigger<OnRemove, SpawnPointEntity>,
    mut commands: Commands,
    query: Query<&SpawnPointEntity>,
) {
    let spawn_point_entity = **query.get(trigger.entity()).unwrap();

    if let Some(mut cmd) = commands.get_entity(spawn_point_entity) {
        cmd.remove::<SpawnPointUsed>();
    }
}

/// Set the spawn point entity as unused in [`SpawnPointParent`] when
/// [`SpawnPointUsed`] is being removed.
///
/// Remove [`SpawnPointEntity`] from the associated spaceship.
fn on_spawn_point_freed(
    trigger: Trigger<OnRemove, SpawnPointUsed>,
    mut commands: Commands,
    q_parents: Query<(&SpawnPoint, &SpawnPointUsed, &Parent)>,
    mut q_spawn_parents: Query<&mut SpawnPointParent>,
) {
    let spawn_point_entity = trigger.entity();
    let Ok((&SpawnPoint(team_type), &SpawnPointUsed(spaceship_entity), parent)) =
        q_parents.get(spawn_point_entity)
    else {
        return;
    };

    // Spawn point no longer exists.
    if let Some(mut cmd) = commands.get_entity(spaceship_entity) {
        cmd.remove::<SpawnPointEntity>();
    }

    if let Ok(mut spawn_parent) = q_spawn_parents.get_mut(parent.get()) {
        // Free as unused.
        spawn_parent[team_type as usize].set_unused(spawn_point_entity);
    }
}

/// Parent entity that holds all the [`SpawnPoint`]s.
/// All spawn points should be a direct [`Children`] of this entity.
#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct SpawnPointParent(#[reflect(ignore)] EntityPools<TeamType>);

/// Spawn point for player spaceships. This should be placed as
/// a child under the entity with the [`SpawnPointParent`] component.
///
/// The spawn point also specifies what team it belongs to.
#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct SpawnPoint(TeamType);

#[derive(
    Component,
    Reflect,
    EnumCount,
    EnumIter,
    AsRefStr,
    IntoStaticStr,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
pub enum TeamType {
    A,
    B,
}

#[derive(Component, Deref)]
pub struct SpawnPointUsed(pub Entity);

/// When a spawn point is being used, the entity shall acquire
/// this component and remember which spawn point it has consumed.
#[derive(Component, Deref)]
pub struct SpawnPointEntity(pub Entity);

/// Cahce the entities of the [`SpawnPointParent`]
/// to their [`RoomId`].
///
/// Local clients should only have 1 cache.
#[derive(Resource, Deref, DerefMut, Default)]
pub struct SpawnPointParentCache(HashMap<RoomId, Entity>);
