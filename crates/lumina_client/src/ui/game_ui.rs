use bevy::prelude::*;

mod objective_area_arrrow;
mod player_stats;
mod score_bar;
mod settings;
mod spaceship_stats;
pub mod timer;

pub use player_stats::get_name;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            spaceship_stats::SpaceshipStatsPlugin,
            settings::SettingsUiPlugin,
            score_bar::ScoreBarUiPlugin,
            player_stats::PlayerStatsPlugin,
            objective_area_arrrow::ObjectiveArrowUiPlugin,
        ));
    }
}
