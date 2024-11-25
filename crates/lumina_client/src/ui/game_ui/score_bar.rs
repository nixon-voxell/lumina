use bevy::prelude::*;
use velyst::prelude::*;

use super::main_ui::GameUi;

pub(super) struct ScoreBarUiPlugin;

impl Plugin for ScoreBarUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, ScoreBarFunc>()
            .init_resource::<ScoreBarFunc>()
            .insert_resource(ScoreBarFunc {
                scores: vec![30, 50],
            });
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "score_bar")]
pub struct ScoreBarFunc {
    scores: Vec<usize>,
}
