use crate::grid_spawning::grid_spawn::Grid;
use bevy::utils::HashMap;
use rand::Rng;
use rand::SeedableRng;

/// Creates a cave map using the Random Walk algorithm.
pub fn generate_random_walk_cave(
    grid: Grid,
    width: usize,
    height: usize,
    seed: u32,
    required_empty_percent: f32,
    border_size: usize,
) -> Grid {
    let mut map = grid.data.clone(); // Copy the map
    perform_random_walk(
        &mut map,
        width,
        height,
        seed,
        required_empty_percent,
        border_size,
    );
    Grid {
        data: map,
        border_size,
    } // Return the new map
}

/// Does the Random Walk to make the cave.
fn perform_random_walk(
    map: &mut HashMap<(usize, usize), i32>,
    width: usize,
    height: usize,
    seed: u32,
    required_empty_percent: f32,
    border_size: usize,
) {
    let total_tiles = (width - 2 * border_size) * (height - 2 * border_size);
    let required_empty_tiles =
        (total_tiles as f32 * required_empty_percent / 100.0).round() as usize;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);

    let (mut current_x, mut current_y) = get_random_start(&mut rng, width, height, border_size);
    let mut empty_tile_count = 1; // Start with one empty tile
    const MAX_ITERATIONS: usize = 10000;

    set_tile(map, current_x, current_y, 0); // Make the start tile empty

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // Directions to move
    let mut iterations = 0; // Count the steps

    while empty_tile_count < required_empty_tiles && iterations < MAX_ITERATIONS {
        let (dx, dy) = directions[rng.gen_range(0..4)];
        let (new_x, new_y) = (
            (current_x as isize + dx) as usize,
            (current_y as isize + dy) as usize,
        );

        if is_within_bounds(new_x, new_y, width, height, border_size) {
            if get_tile(map, new_x, new_y) == 1 {
                // If it's a wall, make it empty
                set_tile(map, new_x, new_y, 0);
                empty_tile_count += 1; // Count the new empty tile
                if empty_tile_count >= required_empty_tiles {
                    break; // Early exit if required empty tiles are achieved
                }
            }
            // Move to the new position
            current_x = new_x;
            current_y = new_y;
        } else {
            iterations += 1; // Count the step if out of bounds
        }
    }
}

/// Gets a random start position inside the map.
fn get_random_start(
    rng: &mut impl Rng,
    width: usize,
    height: usize,
    border_size: usize,
) -> (usize, usize) {
    let x = rng.gen_range(border_size..width - border_size);
    let y = rng.gen_range(border_size..height - border_size);
    (x, y)
}

/// Checks if the position is inside the map.
fn is_within_bounds(x: usize, y: usize, width: usize, height: usize, border_size: usize) -> bool {
    x >= border_size && x < width - border_size && y >= border_size && y < height - border_size
}

/// Gets the value of a tile.
fn get_tile(map: &HashMap<(usize, usize), i32>, x: usize, y: usize) -> i32 {
    *map.get(&(x, y)).unwrap_or(&1)
}

/// Sets the value of a tile.
fn set_tile(map: &mut HashMap<(usize, usize), i32>, x: usize, y: usize, value: i32) {
    map.insert((x, y), value);
}
