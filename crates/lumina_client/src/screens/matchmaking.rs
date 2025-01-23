use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;

use crate::player::LocalPlayerId;
use crate::LocalClientId;

use super::Screen;

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
    mut next_screen_state: ResMut<NextState<Screen>>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
) {
    for _ in lobby_data_evr.read() {
        // Update screen state.
        next_screen_state.set(Screen::MultiplayerLobby);
        // Set local player id to the networked version of player id.
        **local_player_id = PlayerId(**local_client_id);
    }
}
