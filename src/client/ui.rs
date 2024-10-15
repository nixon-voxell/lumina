use bevy::prelude::*;
use bevy_motiongfx::MotionGfxPlugin;

pub(super) mod lobby;
pub(super) mod main_menu;
pub(super) mod splash;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MotionGfxPlugin)
            .init_state::<Screen>()
            .add_plugins((
                splash::SplashUiPlugin,
                main_menu::MainMenuUiPlugin,
                lobby::LobbyUiPlugin,
            ));
    }
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
    // Playing,
    // GameOver,
    // Leaderboard,
    // Tutorial,
    // Credits,
    // Loading,
}
