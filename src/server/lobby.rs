use bevy::prelude::*;
use lightyear::prelude::server::*;

use crate::{
    protocol::{ExitLobby, JoinLobby, Lobbies, Lobby},
    server::spawn_player_entity,
};

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lobbies>().add_systems(
            Update,
            (
                ensure_empty_lobby.before(handle_join_lobby),
                handle_join_lobby,
                handle_exit_lobby,
            ),
        );
    }
}

/// Always make sure that there is an empty lobby for players to join.
fn ensure_empty_lobby(mut lobbies: ResMut<Lobbies>) {
    if !lobbies.has_empty_lobby() {
        lobbies.lobbies.push(Lobby::default());
    }
}

fn handle_join_lobby(
    mut commands: Commands,
    mut join_lobby_evt: EventReader<MessageEvent<JoinLobby>>,
    mut lobbies: ResMut<Lobbies>,
    mut room_manager: ResMut<RoomManager>,
) {
    for join_lobby in join_lobby_evt.read() {
        let client_id = join_lobby.context;
        let lobby_id = join_lobby.message.lobby_id;

        info!("Client {client_id:?} joined lobby {lobby_id:?}");
        let lobby = &mut lobbies.lobbies[lobby_id];
        lobby.players.push(client_id);

        room_manager.add_client(client_id, RoomId(lobby_id as u64));

        if lobby.in_game {
            // If the game has already started, we need to spawn the player entity
            let entity = spawn_player_entity(&mut commands, client_id);
            room_manager.add_entity(entity, RoomId(lobby_id as u64));
        }
    }
}

fn handle_exit_lobby(
    mut exit_lobby_evt: EventReader<MessageEvent<ExitLobby>>,
    mut lobbies: ResMut<Lobbies>,
    mut room_manager: ResMut<RoomManager>,
) {
    for exit_lobby in exit_lobby_evt.read() {
        let client_id = exit_lobby.context;
        let lobby_id = exit_lobby.message.lobby_id;

        info!("Client {client_id:?} exited lobby {lobby_id:?}");
        let lobby = &mut lobbies.lobbies[lobby_id];
        lobby.players.push(client_id);

        room_manager.remove_client(client_id, RoomId(lobby_id as u64));

        if lobby.in_game {
            // TODO: handle leaving mid game
            // If the game has already started, we need to despawn the player entity
            // room_manager.remove_entity(, )
        }
    }
}
