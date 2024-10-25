use bevy::prelude::*;
use bevy_vello::VelloPlugin;
use velyst::{prelude::*, VelystPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VelloPlugin::default()))
        .add_plugins(VelystPlugin::default())
        .register_typst_asset::<HelloWorld>()
        .compile_typst_func::<HelloWorld, MainFunc>()
        .render_typst_func::<MainFunc>()
        .insert_resource(MainFunc {
            width: 1280.0,
            height: 720.0,
            red_height: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_boost_meter)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Update the booster fill state
fn update_boost_meter(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut main_func: ResMut<MainFunc>,
) {
    // If space is pressed, increase the boost bar's height
    if keys.pressed(KeyCode::Space) {
        main_func.red_height += 0.5 * time.delta_seconds() as f64;
        println!("Pressed!");
        if main_func.red_height > 1.0 {
            main_func.red_height = 1.0; // Cap it at 100%
        }
    } else {
        // If space is released, reduce the height
        main_func.red_height -= 0.5 * time.delta_seconds() as f64;
        println!("Released!");
        if main_func.red_height < 0.0 {
            main_func.red_height = 0.0; // Min is 0%
        }
    }
}

// `main` function in Typst with their respective values.
#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")] // name of function in the Typst file
struct MainFunc {
    width: f64,
    height: f64,
    red_height: f64,
}

// Path to the Typst file that you created.
#[derive(TypstPath)]
#[typst_path = "typst/test/game_ui.typ"]
struct HelloWorld;
