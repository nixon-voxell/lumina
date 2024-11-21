use bevy::prelude::*;
use lumina_terrain::prelude::*;

use crate::ui::Screen;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainEntity>()
            // .add_systems(OnEnter(Screen::InGame), generate_terrain)
            .add_systems(OnExit(Screen::InGame), clear_terrain);
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
        let entity = world.spawn_empty().id();
        Self(entity)
    }
}
