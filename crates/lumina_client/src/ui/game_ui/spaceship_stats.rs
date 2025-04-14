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
            .push_to_main_window::<SpaceshipStats, MainFunc, _>(MainWindowSet::Default, always_run)
            .init_resource::<MainFunc>()
            .add_systems(Update, spaceship_stats);
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
            &CollectedLumina,
            Option<&AbilityCooldown>,
            Has<AbilityEffect>,
        ),
        With<SourceEntity>,
    >,
    q_weapons: Query<(&WeaponState, &Weapon, Option<&WeaponReload>), With<SourceEntity>>,
    local_player_info: LocalPlayerInfo,
    mut func: ResMut<MainFunc>,
    time: Res<Time>,
    mut boost_lerp: Local<f64>,
    mut health_lerp: Local<f64>,
) {
    const SMOOTH_LERP: f64 = 4.0;
    let smoth_lerp_dt = SMOOTH_LERP * time.delta_seconds_f64();

    func.data = None;
    let Some((
        max_health,
        health,
        dash_cooldown,
        energy,
        spaceship,
        spaceship_type,
        lumina_collected,
        ability_cooldown,
        is_ability_active,
    )) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_spaceships.get(e).ok())
    else {
        *boost_lerp = 0.0;
        *health_lerp = 0.0;
        return;
    };

    let Some((weapon_state, weapon, weapon_reload)) = local_player_info
        .get(PlayerInfoType::Weapon)
        .and_then(|e| q_weapons.get(e).ok())
    else {
        return;
    };

    let spaceship_type = spaceship_type.as_ref();
    // 0.0 if there is no cooldown, otherwise, calculate it.
    let dash_cooldown = dash_cooldown.map_or_default(|c| calculate_cooldown(c));
    let ability_cooldown = ability_cooldown.map_or_default(|c| calculate_cooldown(c));
    // Calculate magazine reloaded.
    let reload_size = weapon_reload.map_or(weapon_state.magazine(), |r| {
        let reload_percentage = r.elapsed_secs() / r.duration().as_secs_f32();
        (reload_percentage * weapon.magazine_size() as f32) as u32
    });

    *boost_lerp = boost_lerp.lerp(
        (energy.energy / spaceship.energy.max_energy) as f64,
        smoth_lerp_dt,
    );
    *health_lerp = health_lerp.lerp(**health as f64, smoth_lerp_dt);

    func.data = Some(dict! {
        "spaceship_type" => spaceship_type,
        "health" => *health_lerp,
        "max_health" => **max_health as f64,
        "boost" => *boost_lerp,
        "dash_cooldown" => dash_cooldown,
        "ability_cooldown" => ability_cooldown,
        "ability_active" => is_ability_active,
        "magazine" => weapon_state.magazine(),
        "magazine_size" => weapon.magazine_size(),
        "reload_size" => reload_size,
        "lumina_count" => lumina_collected.0,
    });

    func.dummy_update = func.dummy_update.wrapping_add(1);
}

fn calculate_cooldown(timer: &Timer) -> f64 {
    1.0 - timer.elapsed().as_secs_f64() / timer.duration().as_secs_f64()
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")]
struct MainFunc {
    data: Option<Dict>,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/spaceship_stats.typ"]
pub struct SpaceshipStats;
