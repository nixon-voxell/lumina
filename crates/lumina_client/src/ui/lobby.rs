use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use super::Screen;

pub(super) struct LobbyUiPlugin;

impl Plugin for LobbyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyUi>()
            .compile_typst_func::<LobbyUi, LobbyFunc>()
            .recompile_on_interaction::<LobbyFunc>(|func| &mut func.dummy_update)
            .push_to_main_window::<LobbyUi, LobbyFunc, _>(
                MainWindowSet::Default,
                in_state(Screen::MultiplayerLobby),
            )
            .init_resource::<LobbyFunc>()
            .add_systems(
                Update,
                exit_lobby_btn.run_if(in_state(Screen::MultiplayerLobby)),
            )
            .add_systems(Update, (handle_lobby_data, handle_lobby_update));
    }
}

fn exit_lobby_btn(
    interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    if interactions.pressed("btn:exit-lobby") {
        let _ = connection_manager.send_message::<OrdReliableChannel, _>(&ExitLobby);
        next_screen_state.set(Screen::LocalLobby);
    }
}

/// Digest data from [`LobbyUpdate`].
fn handle_lobby_update(
    mut evr_lobby_update: EventReader<MessageEvent<LobbyUpdate>>,
    mut lobby_func: ResMut<LobbyFunc>,
) {
    for lobby_status in evr_lobby_update.read() {
        lobby_func.curr_player_count = lobby_status.message().client_count;
    }
}

/// Digest data from [`LobbyData`]
fn handle_lobby_data(
    mut evr_lobby_data: EventReader<MessageEvent<LobbyData>>,
    mut lobby_func: ResMut<LobbyFunc>,
) {
    for data in evr_lobby_data.read() {
        let data = data.message();

        // Update ui.
        lobby_func.room_id = Some(data.room_id.0);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "lobby", layer = 1)]
pub(super) struct LobbyFunc {
    pub curr_player_count: u8,
    pub max_player_count: u8,
    pub room_id: Option<u64>,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/lobby.typ"]
struct LobbyUi;
