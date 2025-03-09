use bevy::prelude::*;
use blenvy::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin {
                export_registry: false,
                ..default()
            },
        ))
        .run();
}

// fn import_asset(mut commands: Commands) {}
