use bevy::prelude::*;
use bevy_enoki::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

use crate::camera::CameraShake;

use super::LocalPlayerId;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InPlaceVfxMapPlugin::<Weapon>::default())
            .add_systems(Update, (attack_cam_shake, attack_vfx));
    }
}

fn attack_cam_shake(
    mut evr_fire_ammo: EventReader<FireAmmo>,
    local_player_id: Res<LocalPlayerId>,
    mut camera_shake: ResMut<CameraShake>,
) {
    for fire_ammo in evr_fire_ammo.read() {
        if fire_ammo.player_id == **local_player_id {
            camera_shake.add_trauma_with_threshold(0.4, 0.5);
        }
    }
}

fn attack_vfx(
    mut evr_fire_ammo: EventReader<FireAmmo>,
    q_vfx_map: Query<&InPlaceVfxMap>,
    mut q_states: Query<&mut ParticleSpawnerState>,
    player_infos: Res<PlayerInfos>,
) {
    for fire_ammo in evr_fire_ammo.read() {
        let Some(vfx_map) = player_infos[PlayerInfoType::Weapon]
            .get(&fire_ammo.player_id)
            .and_then(|e| q_vfx_map.get(*e).ok())
        else {
            continue;
        };

        for vfx_entity in vfx_map.values().flat_map(|entities| entities.iter()) {
            if let Ok(mut vfx_state) = q_states.get_mut(*vfx_entity) {
                vfx_state.active = true;
            }
        }
    }
}
