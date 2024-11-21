use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;

use super::player::LocalPlayerId;
use super::ui::{lobby::LobbyFunc, Screen};
use super::LocalClientId;

pub(super) struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enter_multiplayer_lobby.run_if(in_state(Screen::Matchmaking)),
        );
    }
}

/// Enter multiplayer lobby
fn enter_multiplayer_lobby(
    mut lobby_data_evr: EventReader<MessageEvent<LobbyData>>,
    mut lobby_func: ResMut<LobbyFunc>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
) {
    for data in lobby_data_evr.read() {
        let data = data.message();

        // Update ui.
        lobby_func.room_id = Some(data.room_id.0);

        // Update screen state.
        next_screen_state.set(Screen::MultiplayerLobby);
        // Set local player id to the networked version of player id.
        **local_player_id = PlayerId(**local_client_id);
    }
}
