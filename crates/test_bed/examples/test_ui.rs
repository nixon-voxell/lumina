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
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// `main` function in Typst with their respective values.
#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")] // name of function in the Typst file
struct MainFunc {
    width: f64,
    height: f64,
}

// Path to the Typst file that you created.
#[derive(TypstPath)]
#[typst_path = "typst/test/test_ui.typ"]
struct HelloWorld;
