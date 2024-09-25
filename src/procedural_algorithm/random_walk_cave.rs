use crate::grid_spawning::grid_spawn::Grid;
use rand::Rng;
use rand::SeedableRng;

/// Generates a cave map using the Random Walk algorithm on an existing map and returns it.
pub fn generate_random_walk_cave(
    grid: Grid,
    width: usize,
    height: usize,
    seed: u32,
    required_empty_percent: f32,
) -> Grid {
    let mut map = grid.0.clone(); // Clone the Vec<i32> from Grid
    random_walk_cave(&mut map, width, height, seed, required_empty_percent);
    Grid(map) // Return the modified Grid
}

/// Generates a cave-like map using the Random Walk Algorithm.
fn random_walk_cave(
    map: &mut Vec<i32>,
    width: usize,
    height: usize,
    seed: u32,
    required_empty_percent: f32,
) {
    let total_tiles = width * height;
    let required_empty_tiles =
        (total_tiles as f32 * required_empty_percent / 100.0).round() as usize;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);

    let (mut floor_x, mut floor_y) = random_start_position(width, height, &mut rng);
    let mut empty_tile_count = 1; // Starting position is already counted as empty
    const MAX_ITERATIONS: usize = 10000;

    set_tile(map, width, floor_x, floor_y, 0); // Set starting position to empty

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut iterations = 0; // Track the number of iterations

    while empty_tile_count < required_empty_tiles && iterations < MAX_ITERATIONS {
        let (dx, dy) = directions[rng.gen_range(0..4)];
        let (new_x, new_y) = (
            (floor_x as isize + dx) as usize,
            (floor_y as isize + dy) as usize,
        );

        if is_within_bounds(new_x, new_y, width, height) {
            if get_tile(map, width, new_x, new_y) == 1 {
                // Check if the tile is a wall
                set_tile(map, width, new_x, new_y, 0); // Convert wall to empty
                empty_tile_count += 1; // Increment empty tile count
            }
            // Update current position
            floor_x = new_x;
            floor_y = new_y;
        } else {
            iterations += 1; // Increment iterations if out of bounds
        }
    }
}

/// Generates a random starting position within the map's bounds.
fn random_start_position(width: usize, height: usize, rng: &mut impl Rng) -> (usize, usize) {
    let x = rng.gen_range(1..width - 1);
    let y = rng.gen_range(1..height - 1);
    (x, y)
}

/// Checks if the given coordinates are within the bounds of the map.
fn is_within_bounds(x: usize, y: usize, width: usize, height: usize) -> bool {
    x > 0 && x < width - 1 && y > 0 && y < height - 1
}

/// Gets the tile value at the specified coordinates.
fn get_tile(map: &[i32], width: usize, x: usize, y: usize) -> i32 {
    map[y * width + x]
}

/// Sets the tile value at the specified coordinates.
fn set_tile(map: &mut [i32], width: usize, x: usize, y: usize, value: i32) {
    map[y * width + x] = value;
}
