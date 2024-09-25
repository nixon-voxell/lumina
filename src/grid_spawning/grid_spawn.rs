use crate::rectangle_spawning::rectangle_entity::{spawn_rectangle, RectangleConfig};
use crate::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Grid(pub Vec<i32>);

impl Grid {
    pub fn new(width: usize, height: usize, fill_value: i32) -> Self {
        // Create a grid filled with the specified fill_value (0 or 1)
        let grid = vec![fill_value; height * width];
        Self(grid)
    }

    // Helper method to access grid elements
    pub fn get(&self, x: usize, y: usize, width: usize) -> i32 {
        self.0[y * width + x]
    }
}

#[derive(Resource)]
pub struct RectangleGridSize {
    pub width: usize,
    pub height: usize,
}

impl RectangleGridSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Default for RectangleGridSize {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}

pub fn spawn_rectangle_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pool: &mut RectanglePool,
    config: RectangleConfig,
    grid_size: &RectangleGridSize,
    grid: &Grid, // Use the grid resource
) {
    let (rect_width, rect_height) = (config.width.value(), config.height.value());

    for y in 0..grid_size.height {
        for x in 0..grid_size.width {
            if grid.get(x, y, grid_size.width) == 1 {
                let position =
                    Transform::from_xyz(x as f32 * rect_width, y as f32 * rect_height, 0.0);
                if let Some(rect_config) = pool.get() {
                    spawn_rectangle(commands, meshes, materials, rect_config, position)
                        .expect("Failed to spawn rectangle");
                }
            }
        }
    }
}
