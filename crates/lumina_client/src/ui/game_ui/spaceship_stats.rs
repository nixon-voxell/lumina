use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};

use crate::player::LocalPlayerId;

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
            &PlayerId,
        ),
        With<SourceEntity>,
    >,
    local_player_id: Res<LocalPlayerId>,
    mut func: ResMut<MainFunc>,
) {
    if let Some((max_health, health, dash_cooldown, energy, spaceship, _)) = q_spaceships
        .iter()
        .find(|(.., &id)| id == local_player_id.0)
    {
        let dash_cooldown = match dash_cooldown {
            Some(dash_cooldown) => 1.0 - dash_cooldown.elapsed_secs() / spaceship.dash.cooldown,
            None => 0.0,
        } as f64;

        func.data = Some(dict! {
            "health" => **health as f64,
            "max_health" => **max_health as f64,
            "boost" => (energy.energy / spaceship.energy.max_energy) as f64,
            "dash_cooldown" => dash_cooldown,
        });
    } else {
        func.data = None;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")]
struct MainFunc {
    data: Option<Dict>,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/spaceship_stats.typ"]
pub struct SpaceshipStats;
