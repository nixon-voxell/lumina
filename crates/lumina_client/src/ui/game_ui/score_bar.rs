use bevy::prelude::*;
use lumina_shared::prelude::*;
use velyst::prelude::*;

use crate::player::{CachedGameStat, LocalPlayerInfo};
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
fn udpate_game_score(
    q_team_types: Query<&TeamType>,
    mut func: ResMut<ScoreBarFunc>,
    game_stat: Res<CachedGameStat>,
    local_player_info: LocalPlayerInfo,
) {
    // Get the local player's team type.
    let Some(team_type) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_team_types.get(e).ok())
    else {
        return;
    };

    if let Some(GameScore {
        score: scores,
        max_score,
    }) = game_stat.game_score
    {
        // Show the local player's score.
        func.score = scores[*team_type as usize].clamp(0, max_score);
        func.max_score = max_score;
    }
}

fn reset_game_score(mut func: ResMut<ScoreBarFunc>) {
    *func = ScoreBarFunc::default();
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "score_bar")]
pub struct ScoreBarFunc {
    // local_score: f64,
    score: u8,
    max_score: u8,
}

impl Default for ScoreBarFunc {
    fn default() -> Self {
        Self {
            score: 0,
            max_score: u8::MAX,
        }
    }
}
