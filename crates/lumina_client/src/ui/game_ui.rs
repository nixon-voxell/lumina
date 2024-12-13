use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

mod score_bar;
mod spaceship_stats;
mod timer;
mod weapon_selector;

use crate::ui::Screen;

use score_bar::ScoreBarFunc;
use timer::CountdownTimerFunc;
use weapon_selector::WeaponSelectorFunc;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            weapon_selector::WeaponSelectorUiPlugin,
            spaceship_stats::SpaceshipStatsPlugin,
            score_bar::ScoreBarUiPlugin,
        ))
        .register_typst_asset::<GameUi>()
        .compile_typst_func::<GameUi, MainFunc>()
        .init_resource::<MainFunc>()
        .add_systems(
            Update,
            (push_to_main_window::<MainFunc>(), game_ui).run_if(in_state(Screen::InGame)),
        );
    }
}

fn game_ui(
    mut func: ResMut<MainFunc>,
    timer_countdown: Res<TypstContent<CountdownTimerFunc>>,
    weapon_selector: Res<TypstContent<WeaponSelectorFunc>>,
    score_bar: Res<TypstContent<ScoreBarFunc>>,
) {
    func.timer = timer_countdown.clone();
    func.weapon_selector = weapon_selector.clone();
    func.score_bar = score_bar.clone();
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    timer: Content,
    weapon_selector: Content,
    score_bar: Content,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_ui.typ"]
struct GameUi;
