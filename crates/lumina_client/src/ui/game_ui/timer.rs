use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::ui::Screen;

pub(super) struct TimerUiPlugin;

impl Plugin for TimerUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<CountdownTimerUi>()
            .compile_typst_func::<CountdownTimerUi, CountdownTimerFunc>()
            .init_resource::<CountdownTimerFunc>()
            .add_systems(Update, start_game)
            .add_systems(
                Update,
                (update_timer, push_to_main_window::<CountdownTimerFunc>())
                    .run_if(in_state(Screen::InGame)),
            );
    }
}

/// Wait for [`StartGame`] command from server.
fn start_game(
    mut evr_start_game: EventReader<MessageEvent<StartGame>>,
    mut timer_func: ResMut<CountdownTimerFunc>,
) {
    for _ in evr_start_game.read() {
        // Allow for custom timing.
        // TODO: Read from a config!
        timer_func.total_seconds = 60.0 * 4.0;
    }
}

/// Update the [`CountdownTimerFunc`].
fn update_timer(time: Res<Time>, mut timer_func: ResMut<CountdownTimerFunc>) {
    // Ensures it stays above 0.0.
    timer_func.total_seconds = (timer_func.total_seconds - time.delta_seconds_f64()).max(0.0);
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
pub struct CountdownTimerFunc {
    pub total_seconds: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/countdown_timer.typ"]
pub struct CountdownTimerUi;
