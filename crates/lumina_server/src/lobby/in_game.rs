use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbyInfos, LobbySeed, TeamType};
use lumina_terrain::map::TerrainStates;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_game, init_spaceship_position));
    }
}

/// Generate terain and start the game when lobby is full.
fn start_game(
    mut commands: Commands,
    q_lobbies: Query<(&LobbySeed, Entity), Added<LobbyFull>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
) {
    for (&LobbySeed(seed), entity) in q_lobbies.iter() {
        generate_terrain_evw.send(GenerateTerrain {
            seed,
            entity,
            layers: CollisionLayers::ALL,
            world_id: PhysicsWorldId(seed),
        });

        // Send message to clients to notify that the game has started.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &StartGame { seed },
            entity.room_id(),
            &room_manager,
        );

        commands.entity(entity).insert(LobbyInGame);
    }
}

/// Updates the position of newly spawned spaceships based on their assigned team type.
fn init_spaceship_position(
    mut commands: Commands,
    mut spaceship_query: Query<(&mut Position, &PlayerId, &TeamType, Entity), With<Spaceship>>,
    in_game_lobbies_query: Query<(), With<LobbyInGame>>,
    terrain_config: TerrainConfig,
    lobby_info: Res<LobbyInfos>,
) {
    // Ensure the terrain config is available
    let Some(terrain_config) = terrain_config.get() else {
        eprintln!("Terrain config is not available!");
        return;
    };

    // Retrieve map corners (bottom-left and upper-right) based on terrain configuration
    let (bottom_left, upper_right) =
        TerrainStates::get_map_corners_without_noise_surr(terrain_config);

    for (mut spaceship_position, player_id, team_type, spaceship_entity) in
        spaceship_query.iter_mut()
    {
        // Skip if the spaceship is not part of an in-game lobby
        if lobby_info
            .get(&**player_id)
            .is_some_and(|lobby_entity| in_game_lobbies_query.contains(*lobby_entity))
            == false
        {
            continue;
        }

        // Position the spaceship based on its assigned team type
        *spaceship_position = match team_type {
            TeamType::A => Position(Vec2::new(bottom_left.x, bottom_left.y)),
            TeamType::B => Position(Vec2::new(upper_right.x, upper_right.y)),
        };

        // Mark the spaceship as initialized
        commands
            .entity(spaceship_entity)
            .insert(PositionInitialized);
    }
}

#[derive(Component)]
pub struct PositionInitialized;
