use bevy::prelude::*;
use bevy_enoki::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

use crate::camera::CameraShake;

use super::LocalPlayerId;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InPlaceVfxMapPlugin::<Weapon>::default())
            .observe(fire_ammo_effect);
    }
}

/// Add particle effects and camera shake on [`FireAmmo`] trigger.
fn fire_ammo_effect(
    trigger: Trigger<FireAmmo>,
    q_weapons: Query<(&InPlaceVfxMap, &PlayerId)>,
    mut q_states: Query<&mut ParticleSpawnerState>,
    local_player_id: Res<LocalPlayerId>,
    mut camera_shake: ResMut<CameraShake>,
) {
    let Ok((vfx_map, player_id)) = q_weapons.get(trigger.event().weapon_entity) else {
        return;
    };

    for vfx_entity in vfx_map.values().flat_map(|entities| entities.iter()) {
        if let Ok(mut vfx_state) = q_states.get_mut(*vfx_entity) {
            vfx_state.active = true;
        }
    }

    if player_id == &**local_player_id {
        camera_shake.add_trauma_with_threshold(0.4, 0.5);
    }
}
