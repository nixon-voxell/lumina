use bevy::prelude::*;

use super::Connection;

mod boostmeter;
mod health;
mod main_ui;
mod score_bar;
mod timer;
mod weapon_selector;

use crate::ui::Screen;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>().add_plugins((
            timer::TimerUiPlugin,
            boostmeter::BoostmeterUiPlugin,
            weapon_selector::WeaponSelectorUiPlugin,
            health::HealthUiPlugin,
            main_ui::MainUiPlugin,
            score_bar::ScoreBarUiPlugin,
        ));

        app.add_systems(OnEnter(Connection::Disconnected), return_to_main_menu);
    }
}

/// Return to main menu
fn return_to_main_menu(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::MainMenu);
}
