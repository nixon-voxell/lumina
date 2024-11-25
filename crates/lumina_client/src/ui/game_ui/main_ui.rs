use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use crate::ui::Screen;

use super::boostmeter::BoostmeterFunc;
use super::health::HealthFunc;
use super::score_bar::ScoreBarFunc;
use super::timer::CountdownTimerFunc;
use super::weapon_selector::WeaponSelectorFunc;

pub(super) struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(
                Update,
                (push_to_main_window::<MainFunc>(), update_main_ui)
                    .run_if(in_state(Screen::Playing)),
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

// `main` function in Typst with their respective values.
#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")] // name of function in the Typst file
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
