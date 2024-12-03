use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use velyst::prelude::*;

use crate::ui::game_ui::GameUi;
use crate::ui::Screen;

pub(super) struct ScoreBarUiPlugin;

impl Plugin for ScoreBarUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, ScoreBarFunc>()
            .init_resource::<ScoreBarFunc>()
            .add_systems(OnEnter(Screen::InGame), reset_game_score)
            .add_systems(Update, udpate_game_score);
    }
}

/// Listen to [`GameScore`] from server.
fn udpate_game_score(
    mut evr_game_score: EventReader<MessageEvent<GameScore>>,
    mut func: ResMut<ScoreBarFunc>,
) {
    for game_score in evr_game_score.read() {
        func.scores = game_score.message().to_vec();
    }
}

fn reset_game_score(mut func: ResMut<ScoreBarFunc>) {
    func.scores = vec![0; 2];
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "score_bar")]
pub struct ScoreBarFunc {
    scores: Vec<u32>,
}
