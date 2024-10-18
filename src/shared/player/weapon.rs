use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use blenvy::*;

use super::BlueprintType;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, init_networked_weapons);

        app.register_type::<WeaponType>().register_type::<Weapon>();
    }
}

// fn attack(
//     q_actions: Query<(&PlayerId, &ActionState<PlayerAction>)>,
//     mut q_spaceships: Query<&PlayerId, With<Weapon>>,
//     time: Res<Time>,
//     player_infos: Res<PlayerInfos>,
// ) {
//     for (id, action) in q_actions.iter() {
//         if action.pressed(&PlayerAction::Attack) {
//             // Shoot ammos!
//         }
//     }
// }

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum WeaponType {
    #[default]
    Cannon,
    Missle,
    GattlingGun,
}

impl BlueprintType for WeaponType {
    fn visual_info(&self) -> BlueprintInfo {
        match self {
            WeaponType::Cannon => BlueprintInfo::from_path("levels/WeaponCannonVisual.glb"),
            _ => todo!("{self:?} is not supported yet."),
        }
    }

    fn config_info(&self) -> BlueprintInfo {
        match self {
            WeaponType::Cannon => BlueprintInfo::from_path("levels/WeaponCannonConfig.glb"),
            _ => todo!("{self:?} is not supported yet."),
        }
    }
}

#[derive(Reflect)]
#[reflect(Component)]
pub struct Weapon {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
    /// Duration the ammo stays relevant.
    ammo_lifetime: f32,
    /// Damage per ammo hit.
    damage: f32,
    /// Recoil force. An impulse force that acts on the opposite of the attack direction.
    recoil: f32,
}

impl Component for Weapon {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let config = world.entity(entity).get::<Self>().unwrap();
            let stat = WeaponStat::from_config(config);
            world.commands().entity(entity).insert(stat);
        });
    }
}

/// The stat of the current weapon.
#[derive(Component)]
pub struct WeaponStat {
    /// Amount of ammo left in the magazine.
    pub magazine: u32,
    /// Accumulated duration since the last attack from the weapon.
    pub recharge: f32,
}

pub struct AmmoStat {
    /// Duration left before the ammo expires.
    pub lifetime: f32,
}

impl WeaponStat {
    pub fn from_config(config: &Weapon) -> Self {
        Self {
            magazine: config.magazine_size,
            recharge: config.firing_rate,
        }
    }
}
