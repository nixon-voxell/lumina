use bevy::asset::AssetMetaCheck;
use bevy::audio::{AudioPlugin, SpatialScale, Volume};
use bevy::prelude::*;
#[cfg(not(feature = "dev"))]
use bevy::window::WindowMode;
use bevy::window::WindowResolution;

fn main() {
    let mut app = App::new();

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
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    #[cfg(not(feature = "dev"))]
                    mode: WindowMode::Fullscreen,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(AudioPlugin {
                default_spatial_scale: SpatialScale::new_2d(0.005),
                global_volume: GlobalVolume {
                    volume: Volume::new(0.3),
                },
            }),
    )
    .add_plugins((
        lumina_common::CommonPlugin,
        lumina_client::ClientPlugin,
        lumina_shared::SharedPlugin,
    ));

    // Disable this in release mode.
    #[cfg(feature = "dev")]
    {
        use bevy::winit::{UpdateMode, WinitSettings};

        app.insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        });
    }

    app.run();
}
