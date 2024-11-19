use bevy::asset::io::Reader;
use bevy::asset::{ron, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(super) struct TerrainConfigPlugin;

impl Plugin for TerrainConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TerrainConfig>()
            .init_asset_loader::<TerrainConfigLoader>()
            .add_systems(PreStartup, load_config);
    }
}

fn load_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("terrain_config.ron");
    commands.insert_resource(TerrainHandle(handle));
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct Terrain<'w> {
    pub assets: Res<'w, Assets<TerrainConfig>>,
    pub handle: Res<'w, TerrainHandle>,
}

impl Terrain<'_> {
    pub fn config(&self) -> Option<&TerrainConfig> {
        self.assets.get(&**self.handle)
    }
}

#[derive(Resource, Deref, Debug)]
pub struct TerrainHandle(Handle<TerrainConfig>);

/// Configuration to generate the terrain procedurally.
#[derive(Asset, TypePath, Deserialize, Serialize, Debug)]
pub struct TerrainConfig {
    /// The size of the terrain (in tile number).
    pub size: UVec2,
    /// Size of a single tile.
    /// This will not interfere with the procedural algorithm in any way.
    pub tile_size: f32,
    /// Width of noise surrounding the map.
    pub noise_surr_width: u32,
    /// Size of the spawn point base.
    pub base_size: UVec2,
    /// Controls the frequency/detail of the noise.
    pub noise_scale: f32,
    /// Any value below this threshold will have a tile placed.
    pub noise_threshold: f32,
    /// The power of gradient from the edges.
    pub gradient_pow: f32,
}

impl AssetLoader for TerrainConfigLoader {
    type Asset = TerrainConfig;
    type Settings = ();
    type Error = TerrainConfigLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let config = ron::de::from_bytes(&bytes)?;

        Ok(config)
    }
}

#[derive(Default)]
pub struct TerrainConfigLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TerrainConfigLoaderError {
    #[error("Could not load json file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not deserialize ron: {0}")]
    Serde(#[from] ron::de::SpannedError),
}
