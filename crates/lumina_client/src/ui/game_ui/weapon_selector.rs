use bevy::prelude::*;
use velyst::prelude::*;

use super::main_ui::GameUi;

pub(super) struct WeaponSelectorUiPlugin;

impl Plugin for WeaponSelectorUiPlugin {
    fn build(&self, app: &mut App) {
        let weapon_configs = WeaponConfigs::default();
        let bullet_counts: Vec<i32> = weapon_configs
            .weapons
            .iter()
            .map(|w| w.current_ammo)
            .collect();

        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, WeaponSelectorFunc>()
            .init_resource::<WeaponSelectorFunc>()
            .insert_resource(weapon_configs)
            .insert_resource(WeaponSelectorFunc {
                selected_index: 0,
                num_weapon: 2,
                bullet_counts,
            })
            .add_systems(Update, (handle_weapon_selection,));
    }
}

#[derive(Resource)]
pub struct WeaponConfigs {
    weapons: Vec<WeaponConfig>,
}

// WeaponConfig struct to store weapon information
#[derive(Clone)]
pub struct WeaponConfig {
    max_ammo: i32,
    current_ammo: i32,
    weapon_type: WeaponType,
}

#[derive(Clone, PartialEq)]
pub enum WeaponType {
    Laser,
    Rifle,
}

impl Default for WeaponConfigs {
    fn default() -> Self {
        Self {
            weapons: vec![
                WeaponConfig {
                    max_ammo: 60,
                    current_ammo: 60,
                    weapon_type: WeaponType::Rifle,
                },
                WeaponConfig {
                    max_ammo: 100,
                    current_ammo: 100,
                    weapon_type: WeaponType::Laser,
                },
            ],
        }
    }
}

fn handle_weapon_selection(
    keys: Res<ButtonInput<KeyCode>>,
    mut weapon_selector: ResMut<WeaponSelectorFunc>,
    weapon_configs: Res<WeaponConfigs>,
) {
    let max_index = weapon_configs.weapons.len() - 1;

    // Can Be Remove: Read max_ammo and weapon_type to avoid dead code warning
    for weapon in &weapon_configs.weapons {
        let _ = weapon.max_ammo;
        let _ = weapon.weapon_type.clone();
    }

    // Handle weapon cycling with Q and E
    if keys.just_pressed(KeyCode::KeyQ) {
        weapon_selector.selected_index = if weapon_selector.selected_index == 0 {
            max_index
        } else {
            weapon_selector.selected_index - 1
        };
    }

    if keys.just_pressed(KeyCode::KeyE) {
        weapon_selector.selected_index = (weapon_selector.selected_index + 1) % (max_index + 1);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "weaponselector")]
pub struct WeaponSelectorFunc {
    selected_index: usize,
    num_weapon: usize,
    bullet_counts: Vec<i32>,
}
