use crate::grid_spawning::grid_spawn::Grid;
use rand::Rng;
use rand::SeedableRng;

pub struct CaveConfig {
    pub map_width: usize,
    pub map_height: usize,
    pub random_seed: u32,
    pub empty_space_percentage: f32,
    pub edge_thickness: usize,
    pub max_dig_attempts: usize,
}

impl Default for CaveConfig {
    fn default() -> Self {
        Self {
            map_width: 100,
            map_height: 100,
            random_seed: 0,
            empty_space_percentage: 40.0,
            edge_thickness: 1,
            max_dig_attempts: 10000,
        }
    }
}

// This function creates a cave-like map using a random walk algorithm
pub fn create_cave_map(initial_grid: Grid, config: CaveConfig) -> Grid {
    let mut cave_map = initial_grid.0.clone();
    carve_cave_paths(&mut cave_map, &config);
    Grid(cave_map)
}

// Choose a random starting position for the cave digging
fn pick_start_position(rng: &mut impl Rng, config: &CaveConfig) -> (usize, usize) {
    let x = rng.gen_range(config.edge_thickness..config.map_width - config.edge_thickness);
    let y = rng.gen_range(config.edge_thickness..config.map_height - config.edge_thickness);
    (x, y)
}

// Check if a position is within the valid bounds
fn is_within_bounds(x: isize, y: isize, config: &CaveConfig) -> bool {
    x >= config.edge_thickness as isize
        && x < (config.map_width - config.edge_thickness) as isize
        && y >= config.edge_thickness as isize
        && y < (config.map_height - config.edge_thickness) as isize
}

// The main function that creates the cave by digging random paths
fn carve_cave_paths(map: &mut [i32], config: &CaveConfig) {
    // Calculate how many empty tiles we need
    let total_tiles = config.map_width * config.map_height;
    let required_empty_tiles =
        (total_tiles as f32 * config.empty_space_percentage / 100.0).round() as usize;

    // Create a random number generator
    let mut rng = rand::rngs::StdRng::seed_from_u64(config.random_seed as u64);

    // Choose a random starting point for digging
    let (mut digger_x, mut digger_y) = pick_start_position(&mut rng, config);

    // Set up variables for the digging process
    let mut empty_tile_count = 1;

    // Make the starting point empty (0 means empty, 1 means wall)
    map[digger_y * config.map_width + digger_x] = 0;

    // Define possible directions to move (up, down, left, right)
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut dig_attempts = 0;

    // Keep digging until we have enough empty tiles or reach max attempts
    while empty_tile_count < required_empty_tiles && dig_attempts < config.max_dig_attempts {
        // Choose a random direction to dig
        let (dx, dy) = directions[rng.gen_range(0..4)];
        let new_x = digger_x as isize + dx;
        let new_y = digger_y as isize + dy;

        // If the new position is valid (inside the map and not in the edge area)
        if is_within_bounds(new_x, new_y, config) {
            let new_index = new_y as usize * config.map_width + new_x as usize;
            // If it's a wall, make it empty
            if map[new_index] == 1 {
                map[new_index] = 0;
                empty_tile_count += 1;
            }
            // Move the digger to the new position
            digger_x = new_x as usize;
            digger_y = new_y as usize;
        } else {
            // If we hit the edge, count it as a dig attempt
            dig_attempts += 1;
        }
    }
}
