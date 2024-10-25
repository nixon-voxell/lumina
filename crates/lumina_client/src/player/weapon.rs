use bevy::prelude::*;
use lumina_shared::player::ammo::FireAmmo;

use crate::camera::CameraShake;

use super::LocalPlayerId;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_cam_shake);
    }
}

fn attack_cam_shake(
    mut fire_ammo_evr: EventReader<FireAmmo>,
    local_player_id: Res<LocalPlayerId>,
    mut camera_shake: ResMut<CameraShake>,
) {
    for fire_ammo in fire_ammo_evr.read() {
        if fire_ammo.id == **local_player_id {
            camera_shake.add_trauma_with_threshold(0.5, 0.2);
        }
    }
}
