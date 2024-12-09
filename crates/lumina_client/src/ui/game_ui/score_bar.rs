use bevy::prelude::*;
use lumina_shared::prelude::*;
use velyst::prelude::*;

use crate::player::CachedGameStat;
use crate::ui::game_ui::GameUi;
use crate::ui::Screen;

pub(super) struct ScoreBarUiPlugin;

impl Plugin for ScoreBarUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, ScoreBarFunc>()
            .init_resource::<ScoreBarFunc>()
            .add_systems(OnEnter(Screen::InGame), reset_game_score)
            .add_systems(
                Update,
                udpate_game_score
                    .run_if(resource_changed::<CachedGameStat>.and_then(in_state(Screen::InGame))),
            );
    }
}

/// Listen to [`GameScore`] from server.
fn udpate_game_score(mut func: ResMut<ScoreBarFunc>, game_stat: Res<CachedGameStat>) {
    if let Some(GameScore { scores, max_score }) = game_stat.game_score {
        func.scores = scores
            .iter()
            .map(|&score| score.clamp(0, max_score))
            .collect();

        func.max_score = max_score;
    }
}

fn reset_game_score(mut func: ResMut<ScoreBarFunc>) {
    *func = ScoreBarFunc::default();
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "score_bar")]
pub struct ScoreBarFunc {
    scores: Vec<u8>,
    max_score: u8,
}

impl Default for ScoreBarFunc {
    fn default() -> Self {
        Self {
            scores: vec![0; 2],
            max_score: u8::MAX,
        }
    }
}
