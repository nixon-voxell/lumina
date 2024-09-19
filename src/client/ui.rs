use bevy::prelude::*;
use lobby::LobbyUiPlugin;
use main_menu::MainMenuUiPlugin;

pub(super) mod lobby;
pub(super) mod main_menu;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuUiPlugin).add_plugins(LobbyUiPlugin);
        // .add_systems(
        //     Update,
        //     (
        //         join_lobby_ui.run_if(in_state(LobbyState::None)),
        //         lobby_ui.run_if(in_state(LobbyState::Joined)),
        //     ),
        // )
        // .add_systems(Update, (connect_server_btn, join_lobby_btn, exit_lobby_btn))
    }
}
