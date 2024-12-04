use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, render::render_resource::ShaderRef};
use bevy_enoki::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::camera::CameraShake;

use super::LocalPlayerId;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Particle2dMaterialPlugin::<MuzzleFlashMaterial>::default())
            .add_systems(Update, (spawn_weapon_vfx, attack_cam_shake, attack_vfx));
    }
}

fn attack_cam_shake(
    mut fire_ammo_evr: EventReader<FireAmmo>,
    local_player_id: Res<LocalPlayerId>,
    mut camera_shake: ResMut<CameraShake>,
) {
    for fire_ammo in fire_ammo_evr.read() {
        if fire_ammo.player_id == **local_player_id {
            camera_shake.add_trauma_with_threshold(0.7, 0.8);
        }
    }
}

fn attack_vfx(
    mut fire_ammo_evr: EventReader<FireAmmo>,
    q_weapon_vfx: Query<&WeaponVfx>,
    mut q_particles: Query<&mut ParticleSpawnerState>,
    q_muzzle_flash_mats: Query<&Handle<MuzzleFlashMaterial>>,
    mut materials: ResMut<Assets<MuzzleFlashMaterial>>,
    player_infos: Res<PlayerInfos>,
) {
    for fire_ammo in fire_ammo_evr.read() {
        let Some(weapon_vfx) = player_infos[PlayerInfoType::Weapon]
            .get(&fire_ammo.player_id)
            .and_then(|e| q_weapon_vfx.get(*e).ok())
        else {
            continue;
        };

        weapon_vfx.activate(&mut q_particles, &q_muzzle_flash_mats, &mut materials);
    }
}

fn spawn_weapon_vfx(
    mut commands: Commands,
    q_weapons: Query<(&Weapon, Entity), Added<SourceEntity>>,
    mut materials: ResMut<Assets<MuzzleFlashMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (weapon, entity) in q_weapons.iter() {
        let muzzle_flash = commands
            .spawn((
                ParticleSpawnerBundle {
                    state: ParticleSpawnerState {
                        active: false,
                        ..default()
                    },
                    effect: asset_server.load("enoki/muzzle_flash.ron"),
                    material: materials.add(MuzzleFlashMaterial::default()),
                    transform: Transform::from_xyz(weapon.fire_radius(), 0.0, 100.0),
                    ..default()
                },
                OneShot::Deactivate,
            ))
            .set_parent(entity)
            .id();

        let gun_sparks = commands
            .spawn((
                ParticleSpawnerBundle {
                    state: ParticleSpawnerState {
                        active: false,
                        ..default()
                    },
                    effect: asset_server.load("enoki/gun_sparks.ron"),
                    material: DEFAULT_MATERIAL,
                    transform: Transform::from_xyz(weapon.fire_radius(), 0.0, 100.0),
                    ..default()
                },
                OneShot::Deactivate,
            ))
            .set_parent(entity)
            .id();

        commands.entity(entity).insert(WeaponVfx {
            muzzle_flash,
            gun_sparks,
        });
    }
}

#[derive(Component)]
struct WeaponVfx {
    muzzle_flash: Entity,
    gun_sparks: Entity,
}

impl WeaponVfx {
    pub fn activate(
        &self,
        q_particles: &mut Query<&mut ParticleSpawnerState>,
        q_muzzle_flash_mats: &Query<&Handle<MuzzleFlashMaterial>>,
        materials: &mut Assets<MuzzleFlashMaterial>,
    ) {
        let entities = [self.muzzle_flash, self.gun_sparks];
        for entity in entities {
            if let Ok(mut particle) = q_particles.get_mut(entity) {
                particle.active = true;
            }
        }

        if let Some(material) = q_muzzle_flash_mats
            .get(self.muzzle_flash)
            .ok()
            .and_then(|handle| materials.get_mut(handle))
        {
            material.variation = rand::random::<f32>() * 9999.9;
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct MuzzleFlashMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    variation: f32,
}

impl Particle2dMaterial for MuzzleFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enoki/muzzle_flash.wgsl".into()
    }
}
