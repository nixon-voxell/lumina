use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

mod score_bar;
mod spaceship_stats;
mod timer;

use crate::ui::Screen;

use score_bar::ScoreBarFunc;
use timer::CountdownTimerFunc;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            spaceship_stats::SpaceshipStatsPlugin,
            score_bar::ScoreBarUiPlugin,
        ))
        .register_typst_asset::<GameUi>()
        .compile_typst_func::<GameUi, MainFunc>()
        .init_resource::<MainFunc>()
        .add_systems(
            Update,
            (push_to_main_window::<MainFunc>(), game_ui)
                .run_if(in_state(Screen::InGame).or_else(in_state(Screen::Sandbox))),
        );
    }
}

fn game_ui(
    mut func: ResMut<MainFunc>,
    timer_countdown: Res<TypstContent<CountdownTimerFunc>>,
    score_bar: Res<TypstContent<ScoreBarFunc>>,
) {
    func.timer = timer_countdown.clone();
    func.score_bar = score_bar.clone();
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    timer: Content,
    score_bar: Content,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_ui.typ"]
struct GameUi;
