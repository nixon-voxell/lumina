use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;

use crate::protocol::LobbyStatus;

use super::{
    ui::{lobby::LobbyFunc, Screen},
    Connection,
};

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MatchmakeState>()
            .add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby)
            .add_systems(
                Update,
                handle_lobby_status_update.run_if(in_state(Connection::Connected)),
            );
    }
}

fn spawn_lobby(mut commands: Commands) {
    commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
}

/// Update [`LobbyFunc`] and [`LobbyState`] based on [`LobbyStatus`].
fn handle_lobby_status_update(
    mut lobby_status_evr: EventReader<MessageEvent<LobbyStatus>>,
    mut lobby_func: ResMut<LobbyFunc>,
    matchmake_state: Res<State<MatchmakeState>>,
    mut next_matchmake_state: ResMut<NextState<MatchmakeState>>,
) {
    for lobby_status in lobby_status_evr.read() {
        let status = lobby_status.message();
        // Update ui
        lobby_func.curr_player_count = status.client_count;
        lobby_func.room_id = Some(status.room_id.0);

        // Update matchmake state
        if *matchmake_state != MatchmakeState::Joined {
            next_matchmake_state.set(MatchmakeState::Joined);
        }
    }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum MatchmakeState {
    #[default]
    None,
    Joining,
    Joined,
    // Starting,
    // Started,
}
