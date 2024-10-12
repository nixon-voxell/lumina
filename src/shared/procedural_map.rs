pub mod grid_map;
pub mod random_walk_cave;

use crate::shared::procedural_map::grid_map::{
    initialize_tile_mesh, setup_grid, spawn_tiles_in_filled_cells, trigger_generate_map_event,
    GenerateMapEvent,
};

use bevy::prelude::*;

pub struct GridMapPlugin;

impl Plugin for GridMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateMapEvent>()
            .add_systems(Startup, initialize_tile_mesh)
            .add_systems(PostStartup, trigger_generate_map_event)
            .add_systems(Update, setup_grid.before(spawn_tiles_in_filled_cells))
            .add_systems(Update, spawn_tiles_in_filled_cells);
    }
}
