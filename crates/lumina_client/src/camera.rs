use avian2d::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::smaa::SmaaSettings;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::window::PrimaryWindow;
use bevy_motiongfx::prelude::*;
use bevy_post_process::chromatic_aberration::ChromaticAberrationConfig;
use bevy_post_process::vignette::VignetteConfig;
use bevy_radiance_cascades::prelude::*;
use bevy_radiance_cascades::radiance_cascades::RadianceCascadesTextures;
use leafwing_input_manager::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use lumina_vfx::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::player::aim::IsUsingMouse;
use crate::player::LocalPlayerId;

use super::player::LocalPlayerInfo;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy_radiance_cascades::FlatlandGiPlugin,
            bevy_post_process::PostProcessPlugin,
        ))
        .init_resource::<CameraZoom>()
        .init_resource::<CameraShake>()
        .add_systems(Startup, spawn_game_camera)
        .add_systems(
            Update,
            (
                health_effect,
                follow_spaceship,
                camera_zoom,
                spaceship_velocity_zoom,
                main_window_zoom.run_if(resource_changed::<MainWindowFunc>),
                propagate_component::<NoRadiance>,
            ),
        )
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
fn spawn_game_camera(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut bloom = BloomSettings::NATURAL;
    bloom.intensity = 0.2;

    let main_camera = commands
        .spawn((
            Name::new("Game Camera"),
            GameCamera,
            RenderLayers::from_layers(&[0, 2]),
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
            VignetteConfig {
                distance: -0.3,
                intensity: 6.0,
                ..default()
            },
            ChromaticAberrationConfig {
                distance: 1.0,
                ..default()
            },
        ))
        .id();

    let window = q_window.single();
    let size = Extent3d {
        width: window.width() as u32,
        height: window.height() as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("main_prepass"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: RadianceCascadesTextures::CASCADE_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        ..default()
    };
    // Fill image.data with zeroes.
    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn((
        Name::new("Main Prepass Camera"),
        MainPrepassCamera(main_camera),
        RenderLayers::layer(0),
        Camera2dBundle {
            camera: Camera {
                order: -1,
                clear_color: Color::NONE.into(),
                target: image_handle.clone_weak().into(),
                hdr: true,
                ..default()
            },
            ..default()
        },
    ));

    commands.insert_resource(MainPrepassTexture::new(image_handle, window.size()));
}

fn health_effect(
    mut commands: Commands,
    mut q_camera: Query<(&mut ChromaticAberrationConfig, &mut VignetteConfig), With<GameCamera>>,
    mut q_spaceships: Query<
        (&Health, Option<&mut PreviousHealth>, &PlayerId, Entity),
        (With<Spaceship>, With<SourceEntity>),
    >,
    local_player_id: Res<LocalPlayerId>,
    time: Res<Time>,
) {
    const SPEED: f32 = 2.0;
    let delta = time.delta_seconds() * SPEED;

    let Ok((mut chrom, mut vignette)) = q_camera.get_single_mut() else {
        return;
    };

    if let Some((health, prev_health, _, entity)) = q_spaceships
        .iter_mut()
        .find(|(.., &id, _)| id == local_player_id.0)
    {
        match prev_health {
            Some(mut prev_health) => {
                if prev_health.0 > **health {
                    // We have been damaged.
                    chrom.distance = -0.4;
                    vignette.tint = Vec3::new(2.0, 0.0, 0.0);
                    // Immediate feedback for damage.
                    prev_health.0 = **health;
                } else if prev_health.0 < **health {
                    // We have been healed.
                    vignette.tint = Vec3::new(0.0, 2.0, 0.0);
                    // Delayed feedback for healing.
                    prev_health.0 = prev_health.0.lerp(**health, time.delta_seconds() * 6.0);

                    if (prev_health.0 - **health).abs() < 0.01 {
                        prev_health.0 = **health;
                    }
                }
            }
            None => {
                commands.entity(entity).insert(PreviousHealth(**health));
            }
        }
    }

    chrom.distance = chrom.distance.lerp(1.0, delta);
    vignette.tint = vignette.tint.lerp(Vec3::ZERO, delta);
}

fn follow_spaceship(
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    q_actions: Query<&ActionState<PlayerAction>, With<SourceEntity>>,
    q_spaceship_transforms: Query<&GlobalTransform, (With<Spaceship>, With<SourceEntity>)>,
    time: Res<Time>,
    local_player_info: LocalPlayerInfo,
    is_using_mouse: Res<IsUsingMouse>,
    mut aim_offset: Local<Vec2>,
) {
    const FOLLOW_FACTOR: f32 = 10.0;
    const AIM_FACTOR: f32 = 2.0;
    const AIM_DISTANCE: f32 = 100.0;

    // Clamp within 1.0 to prevent overshooting
    let aim_factor = f32::min(1.0, AIM_FACTOR * time.delta_seconds());
    let follow_factor = f32::min(1.0, FOLLOW_FACTOR * time.delta_seconds());

    let Some(spaceship_entity) = local_player_info.get(PlayerInfoType::Spaceship) else {
        return;
    };

    // Get local spaceship.
    let Ok(spaceship_translation) = q_spaceship_transforms
        .get(spaceship_entity)
        .map(|t| t.translation())
    else {
        return;
    };

    // Get local action.
    let Some(action) = local_player_info
        .get(PlayerInfoType::Action)
        .and_then(|e| q_actions.get(e).ok())
    else {
        return;
    };

    let mut camera_transform = q_camera.single_mut();

    // Calculate the target position based on player's position.
    let mut target_position = Vec3::new(
        spaceship_translation.x,
        spaceship_translation.y,
        // Keep the same z position.
        camera_transform.translation.z,
    );

    let mut target_aim_offset = Vec2::ZERO;
    if action.pressed(&PlayerAction::Aim) {
        let mut aim_direction = action
            .axis_pair(&PlayerAction::Aim)
            .map(|axis| axis.xy())
            .unwrap_or_default();

        if is_using_mouse.0 {
            aim_direction *= 0.2;
        } else {
            aim_direction *= AIM_DISTANCE;
        }

        target_aim_offset = aim_direction.clamp_length_max(AIM_DISTANCE);
    }

    *aim_offset = Vec2::lerp(*aim_offset, target_aim_offset, aim_factor);
    target_position.x += aim_offset.x;
    target_position.y += aim_offset.y;

    // Smoothly interpolate the camera's position towards the target position.
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_position, follow_factor);
}

fn spaceship_velocity_zoom(
    q_spaceships: Query<(&LinearVelocity, &Spaceship), With<SourceEntity>>,
    mut camera_zoom: ResMut<CameraZoom>,
    local_player_info: LocalPlayerInfo,
) {
    const MAX_ZOOM: f32 = 1.6;

    if let Some((spaceship_velocity, Spaceship { movement, .. })) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_spaceships.get(e).ok())
    {
        // Apply ease to zoom more towards maximal velocity and vice versa.
        let velocity_factor =
            ease::quad::ease_in_out(spaceship_velocity.length() / movement.max_linear_speed)
                .clamp(0.0, 1.0);

        camera_zoom.target_zoom = f32::lerp(1.0, MAX_ZOOM, velocity_factor);
    }
}

fn main_window_zoom(main_window_func: Res<MainWindowFunc>, mut camera_zoom: ResMut<CameraZoom>) {
    const SCALE_MULTIPLIER: f32 = 1.5;

    camera_zoom.zoom_mutliplier = f32::lerp(
        SCALE_MULTIPLIER,
        1.0,
        ease::quad::ease_in_out(main_window_func.transparency as f32),
    );
}

fn camera_zoom(
    mut q_camera: Query<&mut OrthographicProjection, With<GameCamera>>,
    mut camera_zoom: ResMut<CameraZoom>,
    time: Res<Time>,
) {
    const ZOOM_FACTOR: f32 = 4.0;

    let Ok(mut projection) = q_camera.get_single_mut() else {
        return;
    };

    camera_zoom.zoom = f32::lerp(
        camera_zoom.zoom,
        camera_zoom.target_zoom,
        f32::min(1.0, ZOOM_FACTOR * time.delta_seconds()),
    );

    projection.scale = camera_zoom.zoom * camera_zoom.zoom_mutliplier;
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
pub(super) struct CameraZoom {
    zoom: f32,
    zoom_mutliplier: f32,
    target_zoom: f32,
}

impl Default for CameraZoom {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            zoom_mutliplier: 1.0,
            target_zoom: 1.0,
        }
    }
}

#[derive(Resource, Debug)]
pub(super) struct CameraShake {
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

    #[allow(unused)]
    pub fn add_trauma_with_threshold(&mut self, trauma: f32, threshold: f32) {
        // TODO: improve this?
        if self.trauma >= threshold {
            return;
        }

        // How much trauma left can we add.
        let computed_trauma = (threshold - self.trauma).max(0.0);
        // Take the lowest between the amount left and the available amount.
        self.add_trauma(trauma.min(computed_trauma));
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

#[derive(Component, Deref, DerefMut)]
pub struct PreviousHealth(f32);
