use bevy::prelude::*;
use bevy_motiongfx::MotionGfxPlugin;
use velyst::prelude::*;

pub(super) mod lobby;
pub(super) mod main_menu;
pub(super) mod splash;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MotionGfxPlugin)
            .init_state::<Screen>()
            .add_plugins(splash::SplashUiPlugin)
            .add_plugins(main_menu::MainMenuUiPlugin)
            .add_plugins(lobby::LobbyUiPlugin);
    }
}

fn state_scoped_scene<F: TypstFunc>(app: &mut App, state: Screen) {
    app.add_systems(OnEnter(state.clone()), show_scene::<F>)
        .add_systems(OnExit(state), hide_scene::<F>);
}

fn show_scene<F: TypstFunc>(mut scene: ResMut<VelystScene<F>>) {
    scene.visibility = Visibility::Inherited;
}

fn hide_scene<F: TypstFunc>(mut scene: ResMut<VelystScene<F>>) {
    scene.visibility = Visibility::Hidden;
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    // #[default]
    Splash,
    #[default]
    MainMenu,
    MultiplayerLobby,
    // Playing,
    // GameOver,
    // Leaderboard,
    // Tutorial,
    // Credits,
    // Loading,
}
