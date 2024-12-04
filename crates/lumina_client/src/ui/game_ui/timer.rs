use bevy::prelude::*;
use velyst::prelude::*;

use crate::ui::game_ui::GameUi;
use crate::ui::Screen;

pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.compile_typst_func::<GameUi, CountdownTimerFunc>()
            .init_resource::<CountdownTimerFunc>()
            .add_systems(Update, update_timer.run_if(in_state(Screen::InGame)));
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
