use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};
use velyst::typst::utils::OptionExt;

use crate::player::LocalPlayerInfo;

pub(super) struct SpaceshipStatsPlugin;

impl Plugin for SpaceshipStatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SpaceshipStats>()
            .compile_typst_func::<SpaceshipStats, MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(Update, (push_to_main_window::<MainFunc>(), spaceship_stats));
    }
}

fn spaceship_stats(
    q_spaceships: Query<
        (
            &MaxHealth,
            &Health,
            Option<&DashCooldown>,
            &Energy,
            &Spaceship,
            &SpaceshipType,
            Option<&AbilityCooldown>,
            Has<AbilityEffect>,
        ),
        With<SourceEntity>,
    >,
    q_weapons: Query<(&WeaponStat, &Weapon), With<SourceEntity>>,
    local_player_info: LocalPlayerInfo,
    mut func: ResMut<MainFunc>,
) {
    func.data = None;
    let Some((
        max_health,
        health,
        dash_cooldown,
        energy,
        spaceship,
        spaceship_type,
        ability_cooldown,
        is_ability_active,
    )) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_spaceships.get(e).ok())
    else {
        return;
    };

    let Some((weapon_stat, weapon)) = local_player_info
        .get(PlayerInfoType::Weapon)
        .and_then(|e| q_weapons.get(e).ok())
    else {
        return;
    };

    let spaceship_type = spaceship_type.as_ref();
    // 0.0 if there is no cooldown, otherwise, calculate it.
    let dash_cooldown = dash_cooldown.map_or_default(|c| calculate_cooldown(c));
    let ability_cooldown = ability_cooldown.map_or_default(|c| calculate_cooldown(c));

    func.data = Some(dict! {
        "spaceship_type" => spaceship_type,
        "health" => **health as f64,
        "max_health" => **max_health as f64,
        "boost" => (energy.energy / spaceship.energy.max_energy) as f64,
        "dash_cooldown" => dash_cooldown,
        "ability_cooldown" => ability_cooldown,
        "ability_active" => is_ability_active,
        "magazine" => weapon_stat.magazine(),
        "magazine_size" => weapon.magazine_size(),
    });
}

fn calculate_cooldown(timer: &Timer) -> f64 {
    1.0 - timer.elapsed().as_secs_f64() / timer.duration().as_secs_f64()
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")]
struct MainFunc {
    data: Option<Dict>,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/spaceship_stats.typ"]
pub struct SpaceshipStats;
