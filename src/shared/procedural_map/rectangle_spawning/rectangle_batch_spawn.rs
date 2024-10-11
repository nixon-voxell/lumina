use crate::shared::procedural_map::grid::{
    spawn_rectangle_grid, spawn_rectangle_grid_system, Grid, RectangleGridSize,
    SpawnRectangleGridEvent,
};
use crate::shared::procedural_map::random_walk_cave::{create_cave_map, CaveConfig};
use crate::shared::procedural_map::rectangle_spawning::rectangle_entity::{
    RectangleConfig, RectangleMaterialHandle,
};
use crate::shared::procedural_map::rectangle_spawning::rectangle_pool::RectanglePool;
use bevy::prelude::*;
use rand::Rng;

// This struct keeps track of how many rectangles to spawn at once and the queue of rectangles to be spawned.
#[derive(Resource)]
pub struct RectangleBatchSpawner {
    pub batch_size: usize,
    pub progress: usize,
}

impl Default for RectangleBatchSpawner {
    fn default() -> Self {
        Self {
            batch_size: 100,
            progress: 0,
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
        if spawner.progress < spawner.batch_size {
            if let Some(config) = pool.get() {
                spawn_rectangle_grid(
                    commands,
                    meshes,
                    pool,
                    material_handle,
                    config,
                    grid_size,
                    grid,
                );
                spawner.progress += 1;
            } else {
                break; // No more configurations to spawn
            }
        } else {
            break; // Batch size limit reached
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
    mut event_writer: EventWriter<BatchSpawnRectanglesEvent>,
) {
    // Reset the progress
    spawner.progress = 0;

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
    event_writer.send(BatchSpawnRectanglesEvent);
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

        // Create a CaveConfig instance
        let cave_config = CaveConfig {
            map_width: grid_size.width,
            map_height: grid_size.height,
            random_seed: seed,
            empty_space_percentage: required_empty_percent,
            edge_thickness: 1,
            max_dig_attempts: 10000, // or any other value you prefer
        };

        // Generate the cave map using the random walk algorithm
        let cave_map = create_cave_map(initial_map, cave_config);

        app.insert_resource(RectanglePool::new(20_000)) // Corrected
            .insert_resource(RectangleBatchSpawner::default())
            .insert_resource(cave_map) // Insert the generated grid
            .insert_resource(grid_size)
            .insert_resource(RectangleConfig::default()) // Insert the RectangleConfig resource
            .add_event::<SpawnRectangleGridEvent>() // Add the event
            .add_event::<BatchSpawnRectanglesEvent>() // Add the new event
            .add_systems(Startup, preload_rectangles)
            .add_systems(Update, batch_spawn_rectangles)
            .add_systems(Update, spawn_rectangle_grid_system) // Add the system
            .add_systems(Update, handle_batch_spawn_rectangles_event); // Add the new system
    }
}

// This function preloads rectangles into the pool and creates a material handle.
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

// Event to trigger batch spawning of rectangles.
#[derive(Event)]
pub struct BatchSpawnRectanglesEvent;

// This function handles the batch spawn rectangles event.
fn handle_batch_spawn_rectangles_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pool: ResMut<RectanglePool>,
    mut spawner: ResMut<RectangleBatchSpawner>,
    grid_size: Res<RectangleGridSize>,
    grid: Res<Grid>,
    material_handle: Res<RectangleMaterialHandle>,
    mut event_reader: EventReader<BatchSpawnRectanglesEvent>,
) {
    for _ in event_reader.read() {
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
}
