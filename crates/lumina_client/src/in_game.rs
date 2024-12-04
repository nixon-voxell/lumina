use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;

use crate::player::CachedGameStat;
use crate::ui::Screen;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainEntity>()
            .add_systems(OnExit(Screen::InGame), clear_terrain)
            .add_systems(
                Update,
                (
                    update_game_score,
                    game_over.run_if(in_state(Screen::InGame)),
                ),
            );
    }
}

/// Listen to [`GameScore`] from server.
fn update_game_score(
    mut evr_game_score: EventReader<MessageEvent<GameScore>>,
    mut game_stat: ResMut<CachedGameStat>,
) {
    for game_score in evr_game_score.read() {
        game_stat.game_score = Some(game_score.message);
    }
}

/// Listen to [`EndGame`] command.
fn game_over(
    mut evr_end_game: EventReader<MessageEvent<EndGame>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    for _ in evr_end_game.read() {
        // Update screen state.
        next_screen_state.set(Screen::GameOver);
    }
}

fn clear_terrain(
    mut clear_terrain_evw: EventWriter<ClearTerrain>,
    terrain_cache: Res<TerrainEntity>,
) {
    clear_terrain_evw.send(ClearTerrain(**terrain_cache));
}

/// The one and only [`Entity`] that holds all the terrain data in the client.
///
/// See: [lumina_terrain].
#[derive(Resource, Debug, Deref)]
pub struct TerrainEntity(pub Entity);

impl FromWorld for TerrainEntity {
    fn from_world(world: &mut World) -> Self {
        let entity = world.spawn(SpatialBundle::default()).id();
        Self(entity)
    }
}
