use crate::shared::procedural_map::rectangle_spawning::rectangle_entity::{
    spawn_rectangle, RectangleConfig, RectangleMaterialHandle,
};
use crate::shared::procedural_map::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;

// Constants for default values
const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 100;

// Represents a grid of integers
#[derive(Resource, Clone)]
pub struct Grid(pub Vec<i32>);

impl Grid {
    // Creates a new grid filled with a specified value (0 or 1)
    pub fn new(width: usize, height: usize, fill_value: i32) -> Self {
        let grid = vec![fill_value; height * width];
        Self(grid)
    }

    // Helper method to access grid elements
    pub fn get(&self, x: usize, y: usize, width: usize) -> i32 {
        self.0[y * width + x]
    }
}

// Represents the size of the rectangle grid
#[derive(Resource)]
pub struct RectangleGridSize {
    pub width: usize,
    pub height: usize,
}

impl RectangleGridSize {
    // Creates a new RectangleGridSize with specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

// Provides default values for RectangleGridSize
impl Default for RectangleGridSize {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

// Function to spawn rectangles based on the grid
pub fn spawn_rectangle_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    pool: &mut RectanglePool,
    material_handle: &RectangleMaterialHandle,
    config: RectangleConfig,
    grid_size: &RectangleGridSize,
    grid: &Grid,
) {
    let (rect_width, rect_height) = (config.width.value(), config.height.value());

    for y in 0..grid_size.height {
        for x in 0..grid_size.width {
            if grid.get(x, y, grid_size.width) == 1 {
                let position =
                    Transform::from_xyz(x as f32 * rect_width, y as f32 * rect_height, 0.0);
                if let Some(rect_config) = pool.get() {
                    spawn_rectangle(commands, meshes, material_handle, rect_config, position)
                        .expect("Failed to spawn rectangle");
                }
            }
        }
    }
}

// Event to trigger the spawning of the rectangle grid
#[derive(Event)]
pub struct SpawnRectangleGridEvent;

// System that listens for the SpawnRectangleGridEvent and calls the spawn_rectangle_grid function
pub fn spawn_rectangle_grid_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pool: ResMut<RectanglePool>,
    material_handle: Res<RectangleMaterialHandle>,
    config: Res<RectangleConfig>,
    grid_size: Res<RectangleGridSize>,
    grid: Res<Grid>,
    mut event_reader: EventReader<SpawnRectangleGridEvent>,
) {
    // Check if the event was triggered
    for _ in event_reader.read() {
        // Call the spawn_rectangle_grid function
        spawn_rectangle_grid(
            &mut commands,
            &mut meshes,
            &mut pool,
            &material_handle,
            config.clone(),
            &grid_size,
            &grid,
        );
    }
}
