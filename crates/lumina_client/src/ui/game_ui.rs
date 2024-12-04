use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

mod boostmeter;
mod health;
mod score_bar;
mod timer;
mod weapon_selector;

use crate::ui::Screen;

use crate::ui::game_ui::boostmeter::BoostmeterFunc;
use crate::ui::game_ui::health::HealthFunc;
use crate::ui::game_ui::score_bar::ScoreBarFunc;
use crate::ui::game_ui::timer::CountdownTimerFunc;
use crate::ui::game_ui::weapon_selector::WeaponSelectorFunc;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timer::TimerUiPlugin,
            boostmeter::BoostmeterUiPlugin,
            weapon_selector::WeaponSelectorUiPlugin,
            health::HealthUiPlugin,
            score_bar::ScoreBarUiPlugin,
        ))
        .register_typst_asset::<GameUi>()
        .compile_typst_func::<GameUi, MainFunc>()
        .init_resource::<MainFunc>()
        .add_systems(
            Update,
            (push_to_main_window::<MainFunc>(), update_main_ui).run_if(in_state(Screen::InGame)),
        );
    }
}

fn update_main_ui(
    mut func: ResMut<MainFunc>,
    timer_countdown: Res<TypstContent<CountdownTimerFunc>>,
    boostmeter: Res<TypstContent<BoostmeterFunc>>,
    health: Res<TypstContent<HealthFunc>>,
    weapon_selector: Res<TypstContent<WeaponSelectorFunc>>,
    score_bar: Res<TypstContent<ScoreBarFunc>>,
) {
    func.timer = timer_countdown.clone();
    func.boostmeter = boostmeter.clone();
    func.health = health.clone();
    func.weapon_selector = weapon_selector.clone();
    func.score_bar = score_bar.clone();
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    width: f64,
    height: f64,
    boostmeter: Content,
    timer: Content,
    health: Content,
    weapon_selector: Content,
    score_bar: Content,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_ui.typ"]
pub struct GameUi;
