use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_common::prelude::*;

use super::prelude::Spaceship;

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
    spawn_point_query: Query<
        (&GlobalTransform, &TeamType, Entity),
        (With<SpawnPoint>, Without<SpawnPointUsed>),
    >,
    mut spaceship_query: Query<
        (&mut Position, &mut Rotation, Entity),
        (With<Spaceship>, Without<SpawnPointEntity>),
    >,
) {
    let mut available_spawn_points = spawn_point_query.iter();

    for (mut spaceship_position, mut spaceship_rotation, spaceship_entity) in
        spaceship_query.iter_mut()
    {
        // Retrieve the next available spawn point and its transform.
        let Some((spawn_point_transform, team_type, spawn_point_entity)) =
            available_spawn_points.next()
        else {
            return; // Exit if there are no more available spawn points
        };

        // Extract position and rotation from the spawn point's transform
        let (_, spawn_point_rotation, spawn_point_translation) =
            spawn_point_transform.to_scale_rotation_translation();

        // Set spaceship position and rotation based on the spawn point's transform
        *spaceship_position = Position(spawn_point_translation.xy());
        *spaceship_rotation = Rotation::radians(spawn_point_rotation.to_scaled_axis().z);

        // Associate the spaceship with the spawn point, mark it as used, and assign its team type
        commands
            .entity(spaceship_entity)
            .insert((SpawnPointEntity(spawn_point_entity), *team_type)); // Assign the TeamType here
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

#[derive(Component)]
pub struct SpawnPointUsed;

/// When a spawn point is being used, the entity shall acquire
/// this component and remember which spawn point it has consumed.
#[derive(Component, Deref)]
pub struct SpawnPointEntity(pub Entity);
