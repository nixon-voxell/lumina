use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbyInfos, LobbySeed, TeamType};
use lumina_shared::health::{Health, MaxHealth};
use lumina_terrain::map::TerrainStates;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_countdown,
                start_game,
                init_spaceship_position,
                respawn_spaceships,
            ),
        );
    }
}

fn start_countdown(mut commands: Commands, q_lobbies: Query<Entity, Added<LobbyFull>>) {
    for entity in q_lobbies.iter() {
        // Initialize the countdown timer for 5 seconds if it's not already set
        commands.entity(entity).insert(CountdownTimer(5.0));
    }
}

/// Manages the countdown and starts the game for each lobby individually
fn start_game(
    mut commands: Commands,
    mut q_lobbies: Query<(&mut CountdownTimer, &LobbySeed, Entity), With<LobbyFull>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
    time: Res<Time>,
) {
    for (mut countdown_timer, &LobbySeed(seed), entity) in q_lobbies.iter_mut() {
        // Decrease the timer by the actual time elapsed (in seconds)
        countdown_timer.0 -= time.delta_seconds();

        // When the countdown reaches zero, start the game.
        if countdown_timer.0 <= 0.0 {
            // Generate terrain and send messages to notify clients.
            generate_terrain_evw.send(GenerateTerrain {
                seed,
                entity,
                layers: CollisionLayers::ALL,
                world_id: PhysicsWorldId(seed),
            });

            let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
                &StartGame { seed },
                entity.room_id(),
                &room_manager,
            );

            commands
                .entity(entity)
                .insert(LobbyInGame)
                // Remove the countdown timer after the game starts.
                .remove::<CountdownTimer>();

            info!("Game started for lobby {:?} with seed {:?}", entity, seed);
        }
    }
}

/// Updates the position of newly spawned spaceships based on their assigned team type.
fn init_spaceship_position(
    mut commands: Commands,
    mut q_spaceship: Query<
        (&mut Position, &PlayerId, &TeamType, Entity),
        (With<Spaceship>, With<SourceEntity>, Without<InitPosition>),
    >,
    q_in_game_lobbies: Query<(), With<LobbyInGame>>,
    terrain_config: TerrainConfig,
    lobby_info: Res<LobbyInfos>,
) {
    // Ensure the terrain config is available
    let Some(terrain_config) = terrain_config.get() else {
        error!("Terrain config is not available!");
        return;
    };

    // Retrieve map corners (bottom-left and upper-right) based on terrain configuration
    let (bottom_left, upper_right) =
        TerrainStates::get_map_corners_without_noise_surr(terrain_config);

    for (mut spaceship_position, player_id, team_type, spaceship_entity) in q_spaceship.iter_mut() {
        // Skip if the spaceship is not part of an in-game lobby
        if lobby_info
            .get(&**player_id)
            .is_some_and(|lobby_entity| q_in_game_lobbies.contains(*lobby_entity))
            == false
        {
            continue;
        }

        // Determine the position based on team type
        let new_position = match team_type {
            TeamType::A => Vec2::new(bottom_left.x, bottom_left.y),
            TeamType::B => Vec2::new(upper_right.x, upper_right.y),
        };

        *spaceship_position = Position(new_position);

        // Store the initial position in the PositionInitialized component
        commands
            .entity(spaceship_entity)
            .insert(InitPosition(new_position));
    }
}

/// Respawn the spaceship by resetting its position and health to the initial values
pub fn respawn_spaceships(
    mut q_spaceships: Query<
        (
            &mut Position,
            &Visibility,
            &InitPosition,
            &MaxHealth,
            &mut Health,
        ),
        (With<Spaceship>, With<SourceEntity>),
    >,
) {
    for (mut position, visibility, position_initialized, max_health, mut health) in
        q_spaceships.iter_mut()
    {
        // Check if the spaceship is currently hidden
        if *visibility == Visibility::Hidden {
            // Reset position and health.
            position.0 = position_initialized.0;
            **health = **max_health;

            info!(
                "Respawned spaceship at position {:?} with health {:?}",
                position.0, **health
            );
        }
    }
}

/// Initial spawn position of the spaceships.
#[derive(Component)]
pub struct InitPosition(pub Vec2);

/// Countdown before the game starts (in seconds).
#[derive(Component)]
pub struct CountdownTimer(pub f32);
