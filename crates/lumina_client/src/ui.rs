use bevy::prelude::*;

use super::Connection;

pub(super) mod lobby;
pub(super) mod main_menu;
pub(super) mod splash;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>().add_plugins((
            splash::SplashUiPlugin,
            main_menu::MainMenuUiPlugin,
            lobby::LobbyUiPlugin,
        ));

        app.add_systems(OnEnter(Connection::Disconnected), return_to_main_menu);
    }
}

/// Return to main menu
fn return_to_main_menu(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::MainMenu);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    // #[default]
    Splash,
    #[default]
    MainMenu,
    LocalLobby,
    Matchmaking,
    MultiplayerLobby,
    InGame,
    // GameOver,
    // Leaderboard,
    // Tutorial,
    // Credits,
    // Loading,
}
