pub mod config;
pub mod grid_map;
pub mod random_walk_cave;

use blenvy::BlueprintInfo;
use grid_map::{setup_grid_and_spawn_tiles, setup_tile_resources, GenerateMapEvent};

use bevy::prelude::*;
use lumina_common::prelude::*;
use strum::EnumCount;

pub struct GridMapPlugin;

impl Plugin for GridMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(config::TerrainConfigPlugin)
            .init_resource::<EntityPools<TileType>>()
            .add_event::<GenerateMapEvent>()
            .add_systems(Startup, setup_tile_resources)
            .add_systems(Update, setup_grid_and_spawn_tiles);
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct GridMap(Vec2d<bool>);

#[derive(Component, EnumCount, Debug)]
pub enum TileType {
    Tile,
}

impl BlueprintType for TileType {
    fn visual_info(&self) -> BlueprintInfo {
        match self {
            TileType::Tile => BlueprintInfo::from_path("levels/TileVisual.glb"),
            // _ => todo!("{self:?} is not supported yet."),
        }
    }

    fn config_info(&self) -> BlueprintInfo {
        match self {
            TileType::Tile => BlueprintInfo::from_path("levels/TileConfig.glb"),
            // _ => todo!("{self:?} is not supported yet."),
        }
    }
}
