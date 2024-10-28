pub mod grid_map;
pub mod random_walk_cave;

use grid_map::{
    find_valid_spawn_points, initialize_tile_mesh, setup_grid_and_spawn_tiles, GenerateMapEvent,
};

use bevy::prelude::*;

pub struct GridMapPlugin;

impl Plugin for GridMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateMapEvent>()
            .add_systems(Startup, initialize_tile_mesh)
            .add_systems(Update, setup_grid_and_spawn_tiles)
            .add_systems(PostUpdate, find_valid_spawn_points);
    }
}
