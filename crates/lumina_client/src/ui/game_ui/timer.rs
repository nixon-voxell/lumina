use bevy::prelude::*;
use velyst::prelude::*;

use crate::ui::game_ui::GameUi;
pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.compile_typst_func::<GameUi, CountdownTimerFunc>()
            .init_resource::<CountdownTimerFunc>()
            .insert_resource(TimerAccumulator::default())
            .insert_resource(CountdownTimerFunc {
                total_seconds: 90.0,
            })
            .add_systems(Update, update_timer);
    }
}

// A struct to accumulate elapsed time
#[derive(Resource, Default)]
struct TimerAccumulator {
    elapsed: f32, // Accumulated time in seconds
}

/// Update the countdown timer
fn update_timer(
    time: Res<Time>,
    mut timer_func: ResMut<CountdownTimerFunc>,
    mut accumulator: ResMut<TimerAccumulator>,
) {
    // Check if the timer has already reached 0 to avoid unnecessary calculations
    if timer_func.total_seconds <= 0.0 {
        timer_func.total_seconds = 0.0; // Ensure it stays at 0
        return;
    }

    // Accumulate elapsed time
    accumulator.elapsed += time.delta_seconds();

    // Check if a full second has passed
    while accumulator.elapsed >= 1.0 {
        // Decrement the total time by 1 second
        timer_func.total_seconds -= 1.0;

        // Ensure total_seconds doesn't go below 0
        if timer_func.total_seconds <= 0.0 {
            timer_func.total_seconds = 0.0;
            break;
        }

        // Subtract a full second from the accumulator
        accumulator.elapsed -= 1.0;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
pub struct CountdownTimerFunc {
    total_seconds: f64,
}
