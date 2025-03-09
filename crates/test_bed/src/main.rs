use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::smaa::SmaaSettings;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_radiance_cascades::prelude::*;
use blenvy::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin {
                export_registry: false,
                ..default()
            },
            bevy_radiance_cascades::FlatlandGiPlugin,
            lumina_common::CommonPlugin,
        ))
        .add_systems(Startup, (load_blender_scene, setup_camera))
        .run();
}

fn load_blender_scene(mut commands: Commands) {
    commands.spawn((BlueprintInfo::from_path("levels/Scene.glb"), SpawnBlueprint));
}

fn setup_camera(mut commands: Commands) {
    let mut bloom = BloomSettings::NATURAL;
    bloom.intensity = 0.2;

    commands.spawn((
        Name::new("Game Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::NONE.into(),
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                near: -500.0,
                far: 500.0,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: 1280.0,
                    max_height: 720.0,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        bloom,
        SmaaSettings::default(),
        RadianceCascadesConfig::default(),
        SpatialListener::new(400.0),
    ));
}
