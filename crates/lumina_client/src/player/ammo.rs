use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, render::render_resource::ShaderRef};
use bevy_enoki::prelude::*;
use lumina_shared::prelude::*;

use crate::camera::CameraShake;

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Particle2dMaterialPlugin::<AmmoHitMaterial>::default())
            .add_systems(Startup, spawn_ammo_vfx)
            .add_systems(Update, (hit_cam_shake, ammo_hit_vfx));
    }
}

fn hit_cam_shake(mut evr_ammo_hit: EventReader<AmmoHit>, mut camera_shake: ResMut<CameraShake>) {
    for _ in evr_ammo_hit.read() {
        camera_shake.add_trauma_with_threshold(0.3, 0.4);
    }
}

fn ammo_hit_vfx(
    mut q_ammo_hit_vfx: Query<(&mut ParticleSpawnerState, &mut Transform), With<AmmoHitVfx>>,
    mut evr_ammo_hit: EventReader<AmmoHit>,
) {
    let Ok((mut state, mut transform)) = q_ammo_hit_vfx.get_single_mut() else {
        return;
    };

    for ammo_hit in evr_ammo_hit.read() {
        state.active = true;
        // Above walls.
        transform.translation = ammo_hit.extend(10.0);
    }
}

fn spawn_ammo_vfx(
    mut commands: Commands,
    mut materials: ResMut<Assets<AmmoHitMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        ParticleSpawnerBundle {
            state: ParticleSpawnerState {
                active: false,
                ..default()
            },
            effect: asset_server.load("enoki/ammo_hit.ron"),
            material: materials.add(AmmoHitMaterial::default()),
            ..default()
        },
        OneShot::Deactivate,
        AmmoHitVfx,
    ));
}

#[derive(Component)]
struct AmmoHitVfx;

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct AmmoHitMaterial {}

impl Particle2dMaterial for AmmoHitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enoki/ammo_hit.wgsl".into()
    }
}
