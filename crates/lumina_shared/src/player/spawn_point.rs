use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr};

use super::prelude::*;

pub(super) struct SpawnPointPlugin;

impl Plugin for SpawnPointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            init_spaceships_at_spawn_points.after(TransformSystem::TransformPropagate),
        )
        .observe(on_add_spawned)
        .observe(on_remove_spawned);
    }
}

/// Initialize spaceships and assign them to available spawn points.
fn init_spaceships_at_spawn_points(
    mut commands: Commands,
    q_spawn_points: Query<(&GlobalTransform, &SpawnPoint, Entity), Without<SpawnPointUsed>>,
    mut q_spaceship: Query<
        (
            &mut Position,
            &mut Rotation,
            &PlayerId,
            Has<server::SyncTarget>,
            Entity,
        ),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<SpawnPointEntity>,
            Without<TeamType>,
        ),
    >,
) {
    let mut spawn_points = q_spawn_points.iter();

    for (mut position, mut rotation, id, is_server, entity) in q_spaceship.iter_mut() {
        // If we are in multiplayer mode, let the sever handle the spawn position.
        if *id != PlayerId::LOCAL && is_server == false {
            return;
        }

        // Retrieve the next available spawn point and its transform.
        let Some((spawn_transform, spawn_point, spawn_point_entity)) = spawn_points.next() else {
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
            .entity(entity)
            .insert((SpawnPointEntity(spawn_point_entity), **spawn_point));
        commands.entity(spawn_point_entity).insert(SpawnPointUsed); // Mark spawn point as used
    }
}

/// When a [`SpawnPointEntity`] is being added,
/// consume it so that it can't be used again.
fn on_add_spawned(
    trigger: Trigger<OnAdd, SpawnPointEntity>,
    mut commands: Commands,
    query: Query<&SpawnPointEntity>,
) {
    let entity = **query.get(trigger.entity()).unwrap();

    commands.entity(entity).insert(SpawnPointUsed);
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
    let entity = **query.get(trigger.entity()).unwrap();

    if let Some(mut cmd) = commands.get_entity(entity) {
        cmd.remove::<SpawnPointUsed>();
    }
}

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct SpawnPoint(TeamType);

#[derive(
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
    Component,
)]
pub enum TeamType {
    A,
    B,
}

impl TeamType {
    pub fn invert(&self) -> Self {
        match self {
            TeamType::A => TeamType::B,
            TeamType::B => TeamType::A,
        }
    }
}

#[derive(Component)]
pub struct SpawnPointUsed;

/// When a spawn point is being used, the entity shall acquire
/// this component and remember which spawn point it has consumed.
#[derive(Component, Deref)]
pub struct SpawnPointEntity(pub Entity);
