use bevy::prelude::*;
use velyst::prelude::*;

use super::main_ui::GameUi;
pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.compile_typst_func::<GameUi, CountdownTimerFunc>()
            .init_resource::<CountdownTimerFunc>()
            .insert_resource(TimerAccumulator::default())
            .insert_resource(CountdownTimerFunc {
                minutes: "01".to_string(),
                seconds: "30".to_string(),
            })
            .add_systems(Update, update_timer);
    }
}

// A struct to accumulate elapsed time
#[derive(Resource, Default)]
struct TimerAccumulator {
    elapsed: f32, // Accumulated time in seconds
}

// System to update the countdown timer
fn update_timer(
    time: Res<Time>,
    mut timer_func: ResMut<CountdownTimerFunc>,
    mut accumulator: ResMut<TimerAccumulator>, // Use an accumulator to track time
) {
    // Accumulate elapsed time
    accumulator.elapsed += time.delta_seconds();

    // Check if a full second has passed
    if accumulator.elapsed >= 1.0 {
        // Convert strings to integers for manipulation
        let mut minutes = timer_func.minutes.parse::<i64>().unwrap_or(0);
        let mut seconds = timer_func.seconds.parse::<i64>().unwrap_or(0);

        // Reduce the seconds, carrying over to minutes if needed
        if seconds > 0 {
            seconds -= 1;
        } else if minutes > 0 {
            // If seconds reach 0, decrement minutes and reset seconds to 59
            minutes -= 1;
            seconds = 59;
        } else {
            // When both minutes and seconds reach 0, stop the timer
            minutes = 0;
            seconds = 0;
        }

        // Convert the integers back to strings with leading zeros if needed
        timer_func.minutes = format!("{:02}", minutes);
        timer_func.seconds = format!("{:02}", seconds);

        // Subtract the full second that has passed
        accumulator.elapsed -= 1.0;

        // Debug log
        // println!("Timer Updated: {}:{}", timer_func.minutes, timer_func.seconds);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
pub struct CountdownTimerFunc {
    minutes: String,
    seconds: String,
}
