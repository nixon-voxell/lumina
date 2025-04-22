use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};
use velyst::typst::utils::OptionExt;

use crate::player::aim::IsUsingMouse;
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
            Option<&AbilityCooldownTimer>,
            Has<AbilityEffectTimer>,
        ),
        With<SourceEntity>,
    >,
    q_weapons: Query<(&WeaponMagazine, &Weapon, Option<&WeaponReload>), With<SourceEntity>>,
    local_player_info: LocalPlayerInfo,
    mut func: ResMut<MainFunc>,
    time: Res<Time>,
    is_using_mouse: Res<IsUsingMouse>,
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

    let Some((magazine, weapon, weapon_reload)) = local_player_info
        .get(PlayerInfoType::Weapon)
        .and_then(|e| q_weapons.get(e).ok())
    else {
        return;
    };

    let spaceship_type = spaceship_type.as_ref();
    // 0.0 if there is no cooldown, otherwise, calculate it.
    let dash_cooldown = dash_cooldown.map_or_default(|c| calculate_cooldown(c));
    let ability_cooldown = ability_cooldown.map_or_default(|c| calculate_cooldown(c));
    let chunk_size = weapon.reload_chunk_size();
    let chunk_size_f = chunk_size as f32;
    // Number of full chunks in the magazine
    let full_chunks = magazine.0 / chunk_size;
    let reload_size = weapon_reload.map_or(magazine.0 as f32, |r| {
        let base_bullets = (full_chunks * chunk_size) as f32;
        // How far we are through the chunk right now
        let reload_percentage = r.timer.elapsed_secs() / r.timer.duration().as_secs_f32();
        let current_chunk_bullets = reload_percentage * chunk_size_f;
        // Clamp to magazine_size so we never overshoot
        (base_bullets + current_chunk_bullets).min(weapon.magazine_size() as f32)
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
        "magazine" => magazine.0,
        "magazine_size" => weapon.magazine_size(),
        "reload_size" => reload_size as f64,
        "lumina_count" => lumina_collected.0,
        "reload_chunk_size" => chunk_size,
        "full_chunks" => full_chunks,
        "is_using_mouse" => is_using_mouse.0,
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
