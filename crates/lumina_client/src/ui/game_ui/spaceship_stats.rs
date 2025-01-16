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
        (&MaxHealth, &Health, &Boost, &Dash, &PlayerId),
        (With<Spaceship>, With<SourceEntity>),
    >,
    local_player_id: Res<LocalPlayerId>,
    mut func: ResMut<MainFunc>,
) {
    let spaceship = q_spaceships
        .iter()
        .find(|(.., &id)| id == local_player_id.0);

    if let Some((max_health, health, boost, dash, _)) = spaceship {
        func.data = Some(dict! {
            "health" => **health as f64,
            "max_health" => **max_health as f64,
            "boost" => (boost.energy / boost.max_energy) as f64,
            "dash_cooldown" => (dash.current_cooldown / dash.cooldown) as f64,
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
