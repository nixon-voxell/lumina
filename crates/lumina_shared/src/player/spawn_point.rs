use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_common::prelude::*;

use super::prelude::Spaceship;
use lumina_terrain::config::TerrainConfig;
use lumina_terrain::map::TerrainStates;

pub(super) struct SpawnPointPlugin;

impl Plugin for SpawnPointPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TeamAssignment::default()) // Add the team tracking resource.
            .add_systems(
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
    mut team_assignment: ResMut<TeamAssignment>, // Access the team assignment resource.
    q_spawn_points: Query<(&GlobalTransform, Entity), (With<SpawnPoint>, Without<SpawnPointUsed>)>,
    mut q_spaceships: Query<
        (&mut Position, &mut Rotation, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<SpawnPointEntity>,
        ),
    >,
    terrain_config: TerrainConfig,
) {
    if let Some(config) = terrain_config.get() {
        let (bottom_left, upper_right) = TerrainStates::get_map_corners_without_noise_surr(config);

        println!(
            "\n\n\n\n\n Bottom-left corner: x = {}, y = {}",
            bottom_left.x, bottom_left.y
        );
        println!(
            "Upper-right corner: x = {}, y = {}",
            upper_right.x, upper_right.y
        );

        // Define spawn positions for Team A and Team B based on the map corners.
        let team_a_spawn_position = bottom_left; // Use the bottom-left corner for Team A
        let team_b_spawn_position = upper_right; // Use the upper-right corner for Team B

        let mut spawn_points = q_spawn_points.iter();

        for (mut position, mut rotation, entity) in q_spaceships.iter_mut() {
            // Determine the spawn position based on the team assignment.
            let spawn_position = match team_assignment.next_team {
                TeamType::A => team_a_spawn_position,
                TeamType::B => team_b_spawn_position,
            };

            // Set the position for the spaceship based on the team.
            *position = Position(spawn_position);

            // Update the team for the next player.
            team_assignment.next_team = match team_assignment.next_team {
                TeamType::A => TeamType::B,
                TeamType::B => TeamType::A,
            };

            // Obtain a spawn point transform and entity.
            if let Some((spawn_transform, spawn_entity)) = spawn_points.next() {
                // Decompose the transform into scale, rotation, and translation.
                let (_, spawn_rot, _) = spawn_transform.to_scale_rotation_translation();

                // Apply the Z-axis rotation.
                *rotation = Rotation::radians(spawn_rot.to_scaled_axis().z);

                // Add the spawn point entity and mark it as used.
                commands
                    .entity(entity)
                    .insert(SpawnPointEntity(spawn_entity));
                commands.entity(spawn_entity).insert(SpawnPointUsed);
            }
        }
    } else {
        // Handle case if the configuration is not available (optional).
        eprintln!("Terrain config is not available!");
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

// A resource to keep track of which team to assign next.
#[derive(Resource, Default)]
pub struct TeamAssignment {
    pub next_team: TeamType,
}

#[derive(Component)]
pub struct SpawnPointUsed;

/// When a spawn point is being used, the entity shall acquire
/// this component and remember which spawn point it has consumed.
#[derive(Component, Deref)]
pub struct SpawnPointEntity(pub Entity);
