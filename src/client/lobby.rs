use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;

use crate::protocol::LobbyStatus;

use super::{ui::lobby::LobbyFunc, Connection};

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>()
            .enable_state_scoped_entities::<LobbyState>()
            .init_resource::<MyLobbyId>()
            .add_systems(
                Update,
                handle_lobby_status_update.run_if(in_state(Connection::Connected)),
            );
    }
}

/// Update [`LobbyFunc`] and [`LobbyState`] based on [`LobbyStatus`].
fn handle_lobby_status_update(
    mut lobby_status_evr: EventReader<MessageEvent<LobbyStatus>>,
    mut lobby_func: ResMut<LobbyFunc>,
    lobby_state: Res<State<LobbyState>>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    for lobby_status in lobby_status_evr.read() {
        let status = lobby_status.message();
        // Update ui
        lobby_func.curr_player_count = status.client_count;
        lobby_func.room_id = Some(status.room_id.0);

        // Update lobby state
        if *lobby_state != LobbyState::Joined {
            next_lobby_state.set(LobbyState::Joined);
        }
    }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    Joining,
    Joined,
    // InGame,
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub(super) struct MyLobbyId(pub usize);
