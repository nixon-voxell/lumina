use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_common::prelude::*;

use super::prelude::Spaceship;

pub(super) struct SpawnPointPlugin;

impl Plugin for SpawnPointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            init_spaceship.after(TransformSystem::TransformPropagate),
        )
        .observe(on_add_spawned)
        .observe(on_remove_spawned);
    }
}

// Initialize spaceships and assign them alternately to teams.
fn init_spaceship(
    mut commands: Commands,
    q_spawn_points: Query<(&GlobalTransform, Entity), (With<SpawnPoint>, Without<SpawnPointUsed>)>,
    mut q_spaceships: Query<
        (&mut Position, &mut Rotation, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<SpawnPointEntity>,
        ),
    >,
) {
    let mut spawn_points = q_spawn_points.iter();

    for (mut position, mut rotation, entity) in q_spaceships.iter_mut() {
        // Obtain a new spawn transform.
        let Some((spawn_transform, spawn_entity)) = spawn_points.next() else {
            return;
        };

        let (_, spawn_rot, spawn_trans) = spawn_transform.to_scale_rotation_translation();

        *position = Position(spawn_trans.xy());
        *rotation = Rotation::radians(spawn_rot.to_scaled_axis().z);

        commands
            .entity(entity)
            .insert(SpawnPointEntity(spawn_entity));
        commands.entity(spawn_entity).insert(SpawnPointUsed);
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

#[derive(Reflect, PartialEq, Eq, Clone, Copy, Component)]
pub enum TeamType {
    A,
    B,
}

impl Default for TeamType {
    fn default() -> Self {
        TeamType::A // Default team is Team A.
    }
}

#[derive(Component)]
pub struct SpawnPointUsed;

/// When a spawn point is being used, the entity shall acquire
/// this component and remember which spawn point it has consumed.
#[derive(Component, Deref)]
pub struct SpawnPointEntity(pub Entity);
