use bevy::prelude::*;

mod player_stats;
mod score_bar;
mod spaceship_stats;
pub mod timer;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            spaceship_stats::SpaceshipStatsPlugin,
            score_bar::ScoreBarUiPlugin,
            player_stats::PlayerStatsPlugin,
        ));
    }
}
