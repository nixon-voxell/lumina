use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use clap::Parser;

mod client;
mod grid_spawning;
mod procedural_algorithm;
mod protocol;
mod rectangle_spawning;
mod server;
mod settings;
mod shared;
mod ui;
mod utils;

#[cfg(feature = "dev")]
mod dev_tools;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Bevy plugins
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Lumina".to_string(),
                        canvas: Some("Lumina".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        let network_settings_str = include_str!("../network_settings.ron");
        let network_settings =
            settings::read_settings::<settings::NetworkSettings>(network_settings_str);
        app.insert_resource(network_settings);

        let cli = Cli::parse();
        match cli {
            Cli::Server => app.add_plugins(server::ServerPlugin),
            Cli::Client => app.add_plugins(client::ClientPlugin),
        };

        app.add_plugins(shared::SharedPlugin);
        app.add_plugins(rectangle_spawning::rectangle_batch_spawn::RectangleBatchSpawnPlugin);
    }
}

#[derive(Parser, PartialEq, Debug)]
pub enum Cli {
    /// The program will act as server
    Server,
    /// The program will act as a client
    Client,
}
