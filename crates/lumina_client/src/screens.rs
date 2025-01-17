use bevy::prelude::*;

pub(super) mod in_game;
pub(super) mod local_lobby;
pub(super) mod matchmaking;
pub(super) mod multiplayer_lobby;
pub(super) mod sandbox;

pub struct ScreensPlugins;

impl Plugin for ScreensPlugins {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>().add_plugins((
            local_lobby::LocalLobbyPlugin,
            sandbox::SandboxPlugin,
            matchmaking::MatchmakingPlugin,
            multiplayer_lobby::MultiplayerLobbyPlugin,
            in_game::InGamePlugin,
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
    Sandbox,
    Matchmaking,
    MultiplayerLobby,
    InGame,
    GameOver,
    // Leaderboard,
    // Tutorial,
    // Credits,
    // Loading,
}
