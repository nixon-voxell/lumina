use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::game::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::ui::Screen;

pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<CountdownTimerUi>()
            .compile_typst_func::<CountdownTimerUi, CountdownTimerFunc>()
            .push_to_main_window::<CountdownTimerUi, CountdownTimerFunc, _>(
                MainWindowSet::Default,
                in_state(Screen::InGame),
            )
            .init_resource::<CountdownTimerFunc>()
            .add_systems(Update, start_game)
            .add_systems(Update, update_timer.run_if(in_state(Screen::InGame)));
    }
}

/// Wait for [`StartGame`] command from server.
fn start_game(
    mut evr_start_game: EventReader<MessageEvent<StartGame>>,
    mut func: ResMut<CountdownTimerFunc>,
) {
    for _ in evr_start_game.read() {
        func.total_seconds = GAME_DURATION as f64;
    }
}

/// Update the [`CountdownTimerFunc`].
fn update_timer(time: Res<Time>, mut func: ResMut<CountdownTimerFunc>) {
    // Ensures it stays above 0.0.
    func.total_seconds = (func.total_seconds - time.delta_seconds_f64()).max(0.0);
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
pub struct CountdownTimerFunc {
    pub total_seconds: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/countdown_timer.typ"]
pub struct CountdownTimerUi;
