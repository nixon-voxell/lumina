use avian2d::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use noisy_bevy::simplex_noise_2d_seeded;

use super::player::MyPlayer;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShake>()
            .add_systems(Startup, spawn_game_camera)
            .add_systems(Update, follow_player)
            .add_systems(PreUpdate, restore_camera_shake)
            .add_systems(
                PostUpdate,
                camera_shake
                    .before(propagate_transforms)
                    .before(sync_simple_transforms),
            );
    }
}

/// Spawn camera for game rendering (default to render layer 0).
fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        GameCamera,
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::Srgba(Srgba::hex("19181A").unwrap()).into(),
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        BloomSettings::default(),
    ));
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    q_player: Query<&Position, With<MyPlayer>>,
    time: Res<Time>,
) {
    // Adjust this value for more or less delay.
    const LERP_FACTOR: f32 = 3.0;

    // Ensure we have at least one player.
    let Ok(player_pos) = q_player.get_single() else {
        return;
    };

    let mut camera_transform = q_camera.single_mut();

    // Calculate the target position based on player's position.
    let target_position = Vec3::new(
        player_pos.x,
        player_pos.y,
        camera_transform.translation.z, // Keep the same z position
    );

    // Smoothly interpolate the camera's position towards the target position.
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        // Clamp within 1.0 to prevent overshooting
        f32::min(1.0, LERP_FACTOR * time.delta_seconds()),
    );
}

fn restore_camera_shake(
    mut q_cameras: Query<&mut Transform, With<GameCamera>>,
    mut shake: ResMut<CameraShake>,
) {
    for mut transform in q_cameras.iter_mut() {
        // Avoid change detection
        if let Some(reference_translation) = shake.reference_translation {
            transform.translation = reference_translation;
            shake.reference_translation = None;
        }
    }
}

fn camera_shake(
    mut camera: Query<&mut Transform, With<GameCamera>>,
    mut shake: ResMut<CameraShake>,
    time: Res<Time>,
) {
    let mut transform = camera.single_mut();
    shake.reference_translation = Some(transform.translation);

    let translation_offset = Vec3::new(shake.noise_value(0), shake.noise_value(1), 0.0)
        * shake.trauma.powi(2)
        * shake.translation_strength;
    let rotation_offset = Quat::from_rotation_z(
        (shake.noise_value(2) * shake.trauma.powi(2) * shake.rotation_strength).to_radians(),
    );

    transform.translation += translation_offset;
    transform.rotation = Quat::IDENTITY + rotation_offset;

    shake.reduce_trauma(time.delta_seconds());
}

#[derive(Resource, Debug)]
pub struct CameraShake {
    trauma: f32,
    seed: f32,
    noise_strength: f32,
    translation_strength: f32,
    rotation_strength: f32,
    reference_translation: Option<Vec3>,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            seed: 0.0,
            noise_strength: 10.0,
            translation_strength: 25.0,
            rotation_strength: 1.0,
            reference_translation: None,
        }
    }
}

impl CameraShake {
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.trauma == 0.0 {
            self.seed = rand::random();
        }
        self.trauma = (self.trauma + trauma.abs()).min(1.0);
    }

    pub fn add_trauma_with_threshold(&mut self, trauma: f32, threshold: f32) {
        if self.trauma >= threshold {
            return;
        }
        self.add_trauma(trauma);
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.trauma = (self.trauma - delta.abs()).max(0.0)
    }

    fn noise_value(&mut self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.trauma * self.noise_strength, 0.0),
            self.seed + stack as f32,
        )
    }
}

#[derive(Component)]
pub(super) struct GameCamera;
