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
        app.add_systems(Update, (start_game, init_spaceship_position))
            .add_systems(PostUpdate, respawn_spaceships);
    }
}

/// Manages the countdown and starts the game for each lobby individually
fn start_game(
    mut commands: Commands,
    q_lobbies: Query<(&LobbySeed, Entity), Added<LobbyFull>>, // Lobbies that have reached 'full' state
    mut q_timer: Query<(&mut CountdownTimer, Entity)>,        // Timer for each lobby
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
    time: Res<Time>, // Time resource to track elapsed time
) {
    // Loop through all entities (lobbies) with a countdown timer
    for (mut countdown_timer, entity) in q_timer.iter_mut() {
        // Decrease the timer by the actual time elapsed (in seconds)
        countdown_timer.0 -= time.delta_seconds();

        // Debugging: Log the current timer value for this lobby
        info!(
            "Countdown timer for lobby {:?} is at {:.2} seconds",
            entity, countdown_timer.0
        );

        // When the countdown reaches zero, start the game
        if countdown_timer.0 <= 0.0 {
            // Loop through all lobbies that are "full"
            for (&LobbySeed(seed), lobby_entity) in q_lobbies.iter() {
                // Generate terrain and send messages to notify clients
                generate_terrain_evw.send(GenerateTerrain {
                    seed,
                    entity,
                    layers: CollisionLayers::ALL,
                    world_id: PhysicsWorldId(seed),
                });

                // Send message to clients to notify that the game has started.
                let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
                    &StartGame { seed },
                    lobby_entity.room_id(),
                    &room_manager,
                );

                // Update lobby state to "in-game"
                commands.entity(lobby_entity).insert(LobbyInGame);

                // Log that the game is starting for this lobby
                info!("Game started for lobby {:?} with seed {:?}", entity, seed);
            }

            // Remove the countdown timer after the game starts
            commands.entity(entity).remove::<CountdownTimer>();
            info!("Countdown timer removed for lobby {:?}", entity);
        }
    }

    // If the lobby is full and we haven't started the countdown yet, initialize it
    for (_, entity) in q_lobbies.iter() {
        // Initialize the countdown timer for 5 seconds if it's not already set
        if !q_timer.contains(entity) {
            info!("Initializing countdown timer for lobby {:?}", entity);
            commands.entity(entity).insert(CountdownTimer(5.0)); // 5-second countdown
        }
    }
}

/// Updates the position of newly spawned spaceships based on their assigned team type.
fn init_spaceship_position(
    mut commands: Commands,
    mut q_spaceship: Query<
        (&mut Position, &PlayerId, &TeamType, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<PositionInitialized>,
        ),
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
            .insert(PositionInitialized(new_position));
    }
}

/// Respawn the spaceship by resetting its position and health to the initial values
pub fn respawn_spaceships(
    mut q_spaceships: Query<
        (
            &mut Position,
            &mut Visibility,
            &PositionInitialized,
            &MaxHealth,
            &mut Health,
        ),
        (With<Spaceship>, With<SourceEntity>),
    >,
) {
    for (mut position, mut visibility, position_initialized, max_health, mut health) in
        q_spaceships.iter_mut()
    {
        // Check if the spaceship is currently hidden
        if *visibility == Visibility::Hidden {
            // Reset the position to the stored initial position
            position.0 = position_initialized.0;

            // Reset the health to the maximum health value
            health.set(max_health.get());

            // Make the spaceship visible again
            *visibility = Visibility::Inherited;

            info!(
                "Respawned spaceship at position {:?} with health {:?}",
                position.0,
                health.get()
            );
        }
    }
}

#[derive(Component)]
pub struct PositionInitialized(pub Vec2);

/// Component that tracks time remaining for the countdown
#[derive(Component)]
pub struct CountdownTimer(pub f32); // Time in seconds
