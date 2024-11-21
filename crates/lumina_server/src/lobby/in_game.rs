use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbySeed};

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_game);
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
            // TODO: Use 1 -> 32 layers lol.
            layers: CollisionLayers::ALL,
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
