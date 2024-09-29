use crate::rectangle_spawning::rectangle_entity::{
    spawn_rectangle, RectangleConfig, RectangleMaterialHandle,
};
use crate::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Clone)]
pub struct Grid {
    pub data: HashMap<(usize, usize), i32>,
    pub border_size: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize, fill_value: i32, border_size: usize) -> Self {
        let mut grid = HashMap::new();
        if fill_value != 0 {
            for y in border_size..height - border_size {
                for x in border_size..width - border_size {
                    grid.insert((x, y), fill_value);
                }
            }
        }
        Self {
            data: grid,
            border_size,
        }
    }

    // Helper method to access grid elements
    pub fn get(&self, x: usize, y: usize) -> i32 {
        *self.data.get(&(x, y)).unwrap_or(&0)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        if value == 0 {
            self.data.remove(&(x, y));
        } else {
            self.data.insert((x, y), value);
        }
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
    materials: &Res<RectangleMaterialHandle>, // Borrow the value
    pool: &mut RectanglePool,
    config: RectangleConfig,
    grid_size: &RectangleGridSize,
    grid: &Grid, // Use the grid resource
) {
    let (rect_width, rect_height) = (config.width.value(), config.height.value());
    let border_size = grid.border_size;

    for y in border_size..grid_size.height - border_size {
        for x in border_size..grid_size.width - border_size {
            if grid.get(x, y) == 1 {
                let position =
                    Transform::from_xyz(x as f32 * rect_width, y as f32 * rect_height, 0.0);
                if let Some(rect_config) = pool.get() {
                    spawn_rectangle(
                        commands,
                        meshes,
                        materials, // Pass the borrowed value
                        rect_config,
                        position,
                    )
                    .expect("Failed to spawn rectangle");
                }
            }
        }
    }
}
