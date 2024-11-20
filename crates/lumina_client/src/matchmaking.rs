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
            cache_lobby_data.run_if(in_state(Screen::Matchmaking)),
        );
    }
}

/// Cache data from [`LobbyData`].
///
/// When this message is sent, it means that we are officially in multiplayer mode.
fn cache_lobby_data(
    mut commands: Commands,
    mut lobby_data_evr: EventReader<MessageEvent<LobbyData>>,
    mut lobby_func: ResMut<LobbyFunc>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
) {
    for data in lobby_data_evr.read() {
        let data = data.message();
        commands.insert_resource(LobbyDataCache(*data));

        // Update ui.
        lobby_func.room_id = Some(data.room_id.0);

        // Update screen state.
        next_screen_state.set(Screen::MultiplayerLobby);
        // Set local player id to the networked version of player id.
        **local_player_id = PlayerId(**local_client_id);
    }
}

#[derive(Resource, Deref)]
pub struct LobbyDataCache(pub LobbyData);
