use bevy::prelude::*;
use bevy_vello::VelloPlugin;
use velyst::typst_element::prelude::*;
use velyst::{prelude::*, VelystPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VelloPlugin::default()))
        .add_plugins(VelystPlugin::default())
        .register_typst_asset::<HelloWorld>()
        .compile_typst_func::<HelloWorld, MainFunc>()
        .compile_typst_func::<HelloWorld, CountdownTimerFunc>()
        .compile_typst_func::<HelloWorld, BoostmeterFunc>()
        .render_typst_func::<MainFunc>()
        .init_resource::<CountdownTimerFunc>()
        .init_resource::<BoostmeterFunc>()
        .insert_resource(TimerAccumulator::default())
        .insert_resource(CountdownTimerFunc {
            minutes: "01".to_string(),
            seconds: "30".to_string(),
        }) // Set initial time as strings
        .insert_resource(MainFunc {
            width: 1280.0,
            height: 720.0,
            boostmeter: Vec::new(),
            timer: Vec::new(),
        })
        .insert_resource(BoostmeterFunc {
            height: 400.0,
            width: 50.0,
            red_height: 0.0,
        }) // Set initial boostmeter values
        .add_systems(Startup, setup)
        .add_systems(Update, update_timer)
        .add_systems(
            Update,
            update_timer_ui.run_if(resource_changed::<TypstContent<CountdownTimerFunc>>),
        )
        .add_systems(
            Update,
            update_boost_meter_ui.run_if(resource_changed::<TypstContent<BoostmeterFunc>>),
        )
        .add_systems(Update, update_boost_meter)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// A struct to accumulate elapsed time
#[derive(Resource, Default)]
struct TimerAccumulator {
    elapsed: f32, // Accumulated time in seconds
}

fn update_timer_ui(
    mut func: ResMut<MainFunc>,
    timer_countdown: Res<TypstContent<CountdownTimerFunc>>,
) {
    func.timer.clear();
    func.timer.push(timer_countdown.clone());
}

fn update_boost_meter_ui(
    mut func: ResMut<MainFunc>,
    boostmeter: Res<TypstContent<BoostmeterFunc>>,
) {
    func.boostmeter.clear();
    func.boostmeter.push(boostmeter.clone());
}

// Update the booster fill state
fn update_boost_meter(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut boostmeter_func: ResMut<BoostmeterFunc>,
) {
    // If space is pressed, increase the boost bar's height
    if keys.pressed(KeyCode::Space) {
        boostmeter_func.red_height += 0.5 * time.delta_seconds() as f64;
        if boostmeter_func.red_height > 1.0 {
            boostmeter_func.red_height = 1.0; // Cap it at 100%
        }
    } else {
        // If space is released, reduce the height
        boostmeter_func.red_height -= 0.5 * time.delta_seconds() as f64;
        if boostmeter_func.red_height < 0.0 {
            boostmeter_func.red_height = 0.0; // Min is 0%
        }
    }
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

        // Add a debug log
        println!(
            "Timer Updated: {}:{}",
            timer_func.minutes, timer_func.seconds
        );
    }
}

// `main` function in Typst with their respective values.
#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")] // name of function in the Typst file
struct MainFunc {
    width: f64,
    height: f64,
    boostmeter: Vec<Content>,
    timer: Vec<Content>,
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "countdown_timer")]
struct CountdownTimerFunc {
    minutes: String,
    seconds: String,
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "boostmeter")]
struct BoostmeterFunc {
    height: f64,
    width: f64,
    red_height: f64,
}

// Path to the Typst file that you created.
#[derive(TypstPath)]
#[typst_path = "typst/test/game_ui.typ"]
struct HelloWorld;
