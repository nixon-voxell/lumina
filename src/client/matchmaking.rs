use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;

use crate::{protocol::LobbyStatus, shared::player::PlayerId};

use super::{
    ui::{lobby::LobbyFunc, Screen},
    LocalClientId, LocalPlayerId,
};

pub(super) struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Matchmaking), spawn_lobby)
            .add_systems(
                Update,
                handle_lobby_status_update.run_if(in_state(Screen::Matchmaking)),
            );
    }
}

fn spawn_lobby(mut commands: Commands) {
    commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
}

/// Update [`LobbyFunc`] and [`MatchmakeState`] based on [`LobbyStatus`].
fn handle_lobby_status_update(
    mut lobby_status_evr: EventReader<MessageEvent<LobbyStatus>>,
    mut lobby_func: ResMut<LobbyFunc>,
    screen_state: Res<State<Screen>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
) {
    for lobby_status in lobby_status_evr.read() {
        let status = lobby_status.message();
        // Update ui
        lobby_func.curr_player_count = status.client_count;
        lobby_func.room_id = Some(status.room_id.0);

        if *screen_state != Screen::MultiplayerLobby {
            // Update matchmake state
            next_screen_state.set(Screen::MultiplayerLobby);
            // Set local player id to the networked version of player id.
            **local_player_id = PlayerId(**local_client_id);
        }
    }
}
