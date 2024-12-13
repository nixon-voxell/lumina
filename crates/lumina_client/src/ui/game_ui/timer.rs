use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use velyst::prelude::*;

use crate::ui::Screen;

use super::GameUi;

pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.compile_typst_func::<GameUi, CountdownTimerFunc>()
            .init_resource::<CountdownTimerFunc>()
            .add_systems(Update, start_game)
            .add_systems(Update, update_timer.run_if(in_state(Screen::InGame)));
    }
}

/// Wait for [`StartGame`] command from server.
fn start_game(
    mut start_game_evr: EventReader<MessageEvent<StartGame>>,
    mut timer_func: ResMut<CountdownTimerFunc>,
) {
    for _ in start_game_evr.read() {
        // Allow for custom timing.
        timer_func.total_seconds = 60.0 * 2.5;
    }
}

/// Update the [`CountdownTimerFunc`].
fn update_timer(time: Res<Time>, mut timer_func: ResMut<CountdownTimerFunc>) {
    // Check if the timer has already reached 0 to avoid unnecessary calculations
    if timer_func.total_seconds <= 0.0 {
        timer_func.total_seconds = 0.0; // Ensure it stays at 0
        return;
    }

    timer_func.total_seconds -= time.delta_seconds_f64();
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
pub struct CountdownTimerFunc {
    pub total_seconds: f64,
}
