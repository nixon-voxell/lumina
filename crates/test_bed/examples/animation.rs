use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, render::camera::ScalingMode};
use blenvy::*;
use lumina_common::convert_3d_to_2d::Convert3dTo2dPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenvyPlugin {
                export_registry: false,
                ..default()
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            Convert3dTo2dPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, trigger_blueprint_animations)
        .run();
}

pub fn trigger_blueprint_animations(
    blueprint_anims: Query<(&BlueprintAnimationPlayerLink, &BlueprintAnimations)>,
    mut animation_players: Query<&mut AnimationPlayer>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if keycode.just_pressed(KeyCode::Space) {
        for (link, animations) in blueprint_anims.iter() {
            let mut animation_player = animation_players.get_mut(link.0).unwrap();

            for &index in animations.named_indices.values() {
                animation_player.start(index).seek_to(1.0).set_speed(-1.0);
            }

            // for (_, active) in animation_player.playing_animations_mut() {
            //     active.seek_to(1.0);
            // }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/animation_test/Tesseract.glb"),
        SpawnBlueprint,
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // clear_color: Color::NONE.into(),
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
            // tonemapping: Tonemapping::TonyMcMapface,
            // deband_dither: DebandDither::Enabled,
            ..default()
        },
        BloomSettings::default(),
    ));
}
