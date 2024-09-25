use crate::grid_spawning::grid_spawn::{spawn_rectangle_grid, Grid, RectangleGridSize};
use crate::procedural_algorithm::random_walk_cave::generate_random_walk_cave;
use crate::rectangle_spawning::rectangle_entity::RectangleConfig;
use crate::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct RectangleBatchSpawner {
    pub batch_size: usize,
    pub spawn_queue: VecDeque<RectangleConfig>,
}

impl Default for RectangleBatchSpawner {
    fn default() -> Self {
        Self {
            batch_size: 100,
            spawn_queue: VecDeque::new(),
        }
    }
}

pub fn batch_spawn_rectangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut pool: ResMut<RectanglePool>,
    mut spawner: ResMut<RectangleBatchSpawner>,
    grid_size: Res<RectangleGridSize>,
    grid: Res<Grid>,
) {
    // Fill the spawn queue with configurations
    fill_spawn_queue(&mut spawner, &mut pool);

    // Spawn rectangles frame by frame
    spawn_rectangles_frame_by_frame(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut pool,
        &mut spawner,
        &grid_size,
        &grid,
    );
}

fn fill_spawn_queue(spawner: &mut RectangleBatchSpawner, pool: &mut RectanglePool) {
    while spawner.spawn_queue.len() < spawner.batch_size {
        if let Some(config) = pool.get() {
            spawner.spawn_queue.push_back(config);
        } else {
            break; // No more configurations available to spawn
        }
    }
}

fn spawn_rectangles_frame_by_frame(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pool: &mut RectanglePool,
    spawner: &mut RectangleBatchSpawner,
    grid_size: &RectangleGridSize,
    grid: &Grid,
) {
    let spawn_per_frame = 100;

    for _ in 0..spawn_per_frame {
        if let Some(config) = spawner.spawn_queue.pop_front() {
            spawn_rectangle_grid(commands, meshes, materials, pool, config, grid_size, grid);
        } else {
            break; // No more configurations to spawn in this frame
        }
    }
}

pub struct RectangleBatchSpawnPlugin;

impl Plugin for RectangleBatchSpawnPlugin {
    fn build(&self, app: &mut App) {
        let grid_size = RectangleGridSize::new(200, 100);
        let seed = rand::thread_rng().gen_range(0..u32::MAX); // Choose a seed for the random number generator
        let required_empty_percent = 40.0; // Percentage of empty spaces in the cave

        // Create an empty map for the cave generation
        let initial_map = Grid::new(grid_size.width, grid_size.height, 1); // Start with all walls (1s)

        // Generate the cave map using the random walk algorithm
        let cave_map = generate_random_walk_cave(
            initial_map,
            grid_size.width,
            grid_size.height,
            seed,
            required_empty_percent,
        );

        app.insert_resource(RectanglePool::new(200_00))
            .insert_resource(RectangleBatchSpawner::default())
            .insert_resource(cave_map) // Insert the generated grid
            .insert_resource(grid_size)
            .add_systems(Startup, preload_rectangles)
            .add_systems(Update, batch_spawn_rectangles);
    }
}

// Placeholder for preload_rectangles function
fn preload_rectangles(mut pool: ResMut<RectanglePool>) {
    pool.preload(200_00);
}
