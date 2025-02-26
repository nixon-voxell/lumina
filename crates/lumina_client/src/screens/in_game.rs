use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;

use crate::player::CachedGameStat;
use crate::screens::Screen;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(Screen::InGame), despawn_in_game_map)
            .add_systems(
                Update,
                (
                    update_game_score,
                    game_over.run_if(in_state(Screen::InGame)),
                ),
            );
    }
}

/// Listen to [`GameScore`] from server.
fn update_game_score(
    mut evr_game_score: EventReader<MessageEvent<GameScore>>,
    mut game_stat: ResMut<CachedGameStat>,
) {
    for game_score in evr_game_score.read() {
        game_stat.game_score = Some(game_score.message);
    }
}

/// Listen to [`EndGame`] command.
fn game_over(
    mut evr_end_game: EventReader<MessageEvent<EndGame>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    for _ in evr_end_game.read() {
        // Update screen state.
        next_screen_state.set(Screen::GameOver);
    }
}

// TODO: Implement this!
fn despawn_in_game_map() {}
