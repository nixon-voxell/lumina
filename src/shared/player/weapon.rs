use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::shared::input::{InputTarget, PlayerAction};

use super::SpaceShip;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_weapon);

        app.register_type::<WeaponConfig>();
    }
}

fn init_weapon(
    mut commands: Commands,
    q_space_ships: Query<Entity, (With<SpaceShip>, Without<WeaponTarget>)>,
) {
    for entity in q_space_ships.iter() {
        // let config = WeaponConfig::default_rifle();
        let weapon_entity = commands.spawn_empty().id();
        commands.entity(entity).insert(WeaponTarget(weapon_entity));
    }
}

fn attack(
    q_actions: Query<&ActionState<PlayerAction>, With<InputTarget>>,
    mut q_space_ships: Query<&WeaponTarget, With<SpaceShip>>,
    time: Res<Time>,
) {
    for action in q_actions.iter() {
        if action.pressed(&PlayerAction::Attack) {
            // Shoot ammos!
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponConfig {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
    /// Duration the ammo stays relevant before despawning.
    ammo_lifetime: f32,
}

// impl WeaponConfig {
//     pub fn default_rifle() -> Self {
//         Self {
//             firing_rate: 2.0,
//             magazine_size: 10,
//             ammo_lifetime: 1.0,
//         }
//     }
// }

/// The stat of the current weapon.
#[derive(Component)]
pub struct WeaponStat {
    /// Amount of ammo left in the magazine.
    pub magazine: u32,
    /// Accumulated duration since the last attack from the weapon.
    pub recharge: f32,
}

pub struct AmmoStat {
    pub lifetime: f32,
}

impl WeaponStat {
    pub fn from_config(config: &WeaponConfig) -> Self {
        Self {
            magazine: config.magazine_size,
            recharge: config.firing_rate,
        }
    }
}

#[derive(Component, Deref)]
pub struct WeaponTarget(Entity);
