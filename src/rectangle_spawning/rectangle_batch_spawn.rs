use crate::grid_spawning::grid_spawn::{
    spawn_rectangle_grid, spawn_rectangle_grid_system, Grid, RectangleGridSize,
    SpawnRectangleGridEvent,
};
use crate::procedural_algorithm::random_walk_cave::generate_random_walk_cave;
use crate::rectangle_spawning::rectangle_entity::{RectangleConfig, RectangleMaterialHandle};
use crate::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

// This struct keeps track of how many rectangles to spawn at once and the queue of rectangles to be spawned.
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

// This function is called to start the batch spawning process.
pub fn batch_spawn_rectangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pool: ResMut<RectanglePool>,
    mut spawner: ResMut<RectangleBatchSpawner>,
    grid_size: Res<RectangleGridSize>,
    grid: Res<Grid>,
    material_handle: Res<RectangleMaterialHandle>,
) {
    // Fill the spawn queue with configurations
    fill_spawn_queue(&mut spawner, &mut pool);

    // Spawn rectangles frame by frame
    spawn_rectangles_frame_by_frame(
        &mut commands,
        &mut meshes,
        &mut pool,
        &mut spawner,
        &grid_size,
        &grid,
        &material_handle,
    );
}

// This function fills the spawn queue with rectangle configurations from the pool.
fn fill_spawn_queue(spawner: &mut RectangleBatchSpawner, pool: &mut RectanglePool) {
    while spawner.spawn_queue.len() < spawner.batch_size {
        if let Some(config) = pool.get() {
            spawner.spawn_queue.push_back(config);
        } else {
            break; // No more configurations available to spawn
        }
    }
}

// This function spawns rectangles frame by frame from the spawn queue.
fn spawn_rectangles_frame_by_frame(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    pool: &mut RectanglePool,
    spawner: &mut RectangleBatchSpawner,
    grid_size: &RectangleGridSize,
    grid: &Grid,
    material_handle: &RectangleMaterialHandle,
) {
    let spawn_per_frame = 100;

    for _ in 0..spawn_per_frame {
        if let Some(config) = spawner.spawn_queue.pop_front() {
            spawn_rectangle_grid(
                commands,
                meshes,
                pool,
                material_handle,
                config,
                grid_size,
                grid,
            );
        } else {
            break; // No more configurations to spawn in this frame
        }
    }
}

// This plugin sets up the batch spawning system.
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

        app.insert_resource(RectanglePool::new(20_000)) // Corrected
            .insert_resource(RectangleBatchSpawner::default())
            .insert_resource(cave_map) // Insert the generated grid
            .insert_resource(grid_size)
            .insert_resource(RectangleConfig::default()) // Insert the RectangleConfig resource
            .add_event::<SpawnRectangleGridEvent>() // Add the event
            .add_systems(Startup, preload_rectangles)
            .add_systems(Update, batch_spawn_rectangles)
            .add_systems(Update, spawn_rectangle_grid_system); // Add the system
    }
}

// Placeholder for preload_rectangles function
fn preload_rectangles(
    mut pool: ResMut<RectanglePool>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    pool.preload(20_000);

    // Create and store the material handle
    let material_handle = materials.add(ColorMaterial::from(Color::srgb(0.0, 0.5, 0.8)));
    commands.insert_resource(RectangleMaterialHandle(material_handle));
}
