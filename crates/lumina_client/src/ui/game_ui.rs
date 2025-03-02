use bevy::prelude::*;

mod score_bar;
mod spaceship_stats;
mod timer;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            spaceship_stats::SpaceshipStatsPlugin,
            score_bar::ScoreBarUiPlugin,
        ));
    }
}
