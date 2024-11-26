use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbyInfos, LobbySeed};

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

/// This function updates the position of newly spawned spaceships
fn init_spaceship_position(
    mut commands: Commands, // Need Commands to add Position if missing
    mut q_spaceships: Query<
        (&mut Position, &PlayerId, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<PositionInitialized>,
        ),
    >,
    q_in_game_lobbies: Query<(), With<LobbyInGame>>,
    terrain_config: TerrainConfig,
    lobby_infos: Res<LobbyInfos>,
) {
    let Some(terrain_config) = terrain_config.get() else {
        return;
    };

    // TODO: Use different positions for different ships based on their team id.
    let width = terrain_config.tile_size * terrain_config.noise_surr_width as f32;
    let desired_position = Vec2::splat(width);

    for (mut position, id, entity) in q_spaceships.iter_mut() {
        if lobby_infos
            .get(&**id)
            .is_some_and(|e| q_in_game_lobbies.contains(*e))
            == false
        {
            continue;
        }

        *position = Position(desired_position);
        commands.entity(entity).insert(PositionInitialized);
    }
}

#[derive(Component)]
pub struct PositionInitialized;
