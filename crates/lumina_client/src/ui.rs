use bevy::prelude::*;
use bevy_radiance_cascades::prelude::*;
use bevy_vello::render::VelloCanvasMaterial;

use crate::screens::Screen;

use super::Connection;

pub(super) mod game_mode;
pub(super) mod game_over;
pub(super) mod game_ui;
pub(super) mod kill_cue;
pub(super) mod lobby;
pub(super) mod main_menu;
pub(super) mod movement_button;
pub(super) mod respawn_cue;
pub(super) mod sandbox;
pub(super) mod spaceship_select;
pub(super) mod splash;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            splash::SplashUiPlugin,
            movement_button::MovementButtonUiPlugin,
            main_menu::MainMenuUiPlugin,
            game_mode::GameModeUiPlugin,
            lobby::LobbyUiPlugin,
            game_ui::GameUiPlugin,
            game_over::GameOverUiPlugin,
            sandbox::SandboxUiPlugin,
            spaceship_select::SpaceshipSelectUiPlugin,
            respawn_cue::RespawnCueUiPlugin,
            kill_cue::KillCueUiPlugin,
        ));

        app.add_systems(OnEnter(Connection::Disconnected), return_to_main_menu)
            .add_systems(Update, set_no_radiance);
    }
}

/// Return to main menu
fn return_to_main_menu(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::MainMenu);
}

// Set all vello canvas to be ignored by radiancs cascades global illumination.
fn set_no_radiance(
    mut commands: Commands,
    q_scenes: Query<Entity, (With<Handle<VelloCanvasMaterial>>, Without<NoRadiance>)>,
) {
    for entity in q_scenes.iter() {
        commands.entity(entity).insert(NoRadiance);
    }
}
