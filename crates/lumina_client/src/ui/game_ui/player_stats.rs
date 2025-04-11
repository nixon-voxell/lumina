use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};

use crate::camera::GameCamera;
use crate::player::CachedGameStat;

pub(super) struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<PlayerStats>()
            .compile_typst_func::<PlayerStats, MainFunc>()
            .push_to_main_window::<PlayerStats, MainFunc, _>(MainWindowSet::Default, always_run)
            .init_resource::<MainFunc>()
            .add_systems(PostUpdate, player_stats.in_set(MainWindowTransformSyncSet));
    }
}

fn player_stats(
    q_spaceships: Query<
        (&TeamType, &PlayerId, &GlobalTransform),
        (With<Spaceship>, With<SourceEntity>),
    >,
    q_game_camera: Query<(&GlobalTransform, &OrthographicProjection, &Camera), With<GameCamera>>,
    game_stat: Res<CachedGameStat>,
    mut func: ResMut<MainFunc>,
) {
    func.players.clear();

    let CachedGameStat {
        team_type: Some(local_team_type),
        ..
    } = *game_stat
    else {
        return;
    };

    let Ok((camera_transform, proj, camera)) = q_game_camera.get_single() else {
        return;
    };
    func.scale = proj.scale as f64;

    for (team_type, id, transform) in q_spaceships.iter() {
        let name = NAMES[id.to_bits() as usize % NAMES.len()];
        let translation = camera
            .world_to_viewport(camera_transform, transform.translation())
            .unwrap_or_default();
        func.players.push(dict! {
            "is_local" => *team_type == local_team_type,
            "x" => translation.x as f64,
            "y" => translation.y as f64,
            "name" => name,
        });
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 0)]
struct MainFunc {
    players: Vec<Dict>,
    scale: f64,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/player_stats.typ"]
pub struct PlayerStats;

const NAMES: [&str; 50] = [
    "Luminara",
    "Gloam",
    "Radiant",
    "Obscura",
    "Lumis",
    "Eclipse",
    "Flare",
    "Nebule",
    "Photon",
    "Umbra",
    "Glimmer",
    "Dusk",
    "Aura",
    "Nova",
    "Eclipse",
    "Brilliance",
    "Cimmer",
    "Prism",
    "Dawn",
    "Gleam",
    "Strobe",
    "Halo",
    "Penumbra",
    "Chroma",
    "Solara",
    "Shimmer",
    "Tenebris",
    "Zenith",
    "Corona",
    "Mirage",
    "Flicker",
    "Oscura",
    "Twilight",
    "Luster",
    "Ember",
    "Glow",
    "Luxis",
    "Eclipse",
    "Radiance",
    "Silhouette",
    "Gleam",
    "Flash",
    "Scintilla",
    "Darken",
    "Shine",
    "Solis",
    "Shadow",
    "Flarescape",
    "Obscurial",
    "Luminae",
];
