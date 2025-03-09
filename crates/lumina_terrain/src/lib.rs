use avian2d::prelude::*;
use bevy::prelude::*;
use config::TerrainConfig;
use lumina_common::prelude::*;
use map::{Terrain, TileRef};
use strum::EnumCount;

pub mod config;
pub mod map;

pub mod prelude {
    pub use crate::config::TerrainConfig;
    pub use crate::map::{Terrain, TerrainStates, TerrainTiles, TileRef};
    pub use crate::{ClearTerrain, GenerateTerrain};
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityPools<TerrainType>>()
            .add_event::<GenerateTerrain>()
            .add_event::<ClearTerrain>()
            .add_plugins(config::TerrainConfigPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (clear_terrain, generate_terrain).chain());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    palette: Res<ColorPalette>,
) {
    commands.insert_resource(TileRef {
        mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        material: materials.add(palette.base1),
        collider: Collider::rectangle(1.0, 1.0),
    })
}

fn clear_terrain(mut terrain: Terrain, mut evr_clear: EventReader<ClearTerrain>) {
    for clear in evr_clear.read() {
        terrain.clear_terrain(**clear);
    }
}

fn generate_terrain(
    mut terrain: Terrain,
    config: TerrainConfig,
    mut evr_generate: EventReader<GenerateTerrain>,
    mut queue: Local<Vec<GenerateTerrain>>,
) {
    // Use a queuing system so that we don't miss any events if the below assets is not ready yet.
    for gen in evr_generate.read() {
        queue.push(*gen);
    }

    let Some(config) = config.get() else {
        return;
    };

    for gen in queue.drain(..) {
        terrain.clear_terrain(gen.entity);
        terrain.generate_terrain(gen.entity, config, &gen);
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct GenerateTerrain {
    pub seed: u32,
    /// The entity that is supposed to hold the [`map::TerrainMapBundle`].
    pub entity: Entity,
    pub layers: CollisionLayers,
    pub world_id: WorldIdx,
}

#[derive(Event, Debug, Deref, Clone, Copy)]
pub struct ClearTerrain(pub Entity);

#[derive(EnumCount)]
pub enum TerrainType {
    Tile,
}
