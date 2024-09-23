use bevy::prelude::*;
use lobby::LobbyUiPlugin;
use main_menu::MainMenuUiPlugin;

pub(super) mod lobby;
pub(super) mod main_menu;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuUiPlugin).add_plugins(LobbyUiPlugin);
    }
}
