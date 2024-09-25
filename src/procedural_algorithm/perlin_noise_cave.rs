// use noise::{NoiseFn, Perlin};
// use std::time::Instant;

// #[allow(dead_code)]
// // Main function to generate a cave map using Perlin noise.
// pub fn generate_perlin_noise_cave(
//     width: usize,
//     height: usize,
//     modifier: f32,
//     edges_are_walls: bool,
//     seed: u32,
//     threshold: f64,
//     wall_value: i32,
//     empty_value: i32,
// ) -> Vec<Vec<i32>> {
//     let start_time = Instant::now(); // Start timing

//     let perlin = create_perlin_noise(seed); // Create noise generator
//     let mut map = initialize_map(width, height, empty_value); // Initialize map
//     fill_map_with_noise(
//         &mut map,
//         width,
//         height,
//         modifier,
//         &perlin,
//         threshold,
//         wall_value,
//         empty_value,
//     ); // Fill with noise

//     if edges_are_walls {
//         set_edges(&mut map, width, height, wall_value); // Set edges if required
//     }

//     let duration = start_time.elapsed(); // Calculate elapsed time
//     println!("Map generation took: {:?}", duration); // Log duration

//     map // Return the completed map
// }

// // Creates a Perlin noise generator using a seed.
// fn create_perlin_noise(seed: u32) -> Perlin {
//     Perlin::new(seed)
// }

// // Initializes a map filled with a specific value.
// fn initialize_map(width: usize, height: usize, empty_value: i32) -> Vec<Vec<i32>> {
//     vec![vec![empty_value; height]; width] // Directly create a 2D vector
// }

// // Fills the map with noise values to create walls and empty spaces.
// fn fill_map_with_noise(
//     map: &mut Vec<Vec<i32>>,
//     width: usize,
//     height: usize,
//     modifier: f32,
//     perlin: &Perlin,
//     threshold: f64,
//     wall_value: i32,
//     empty_value: i32,
// ) {
//     let mod_f64 = modifier as f64; // Convert modifier for calculations

//     // Fill the map using noise directly
//     for x in 0..width {
//         let scaled_x = x as f64 * mod_f64; // Stretch the x position
//         for y in 0..height {
//             let scaled_y = y as f64 * mod_f64; // Stretch the y position
//             let noise_value = perlin.get([scaled_x, scaled_y]); // Get noise value directly
//             map[x][y] = if noise_value > threshold {
//                 wall_value
//             } else {
//                 empty_value
//             };
//         }
//     }
// }

// // Sets the edges of the map to wall values, with edge case handling.
// fn set_edges(map: &mut Vec<Vec<i32>>, width: usize, height: usize, wall_value: i32) {
//     if width == 0 || height == 0 {
//         return; // No map to set edges for
//     }

//     // Set top and bottom edges if height allows
//     if height > 1 {
//         for x in 0..width {
//             map[x][0] = wall_value; // Top edge
//             map[x][height - 1] = wall_value; // Bottom edge
//         }
//     }

//     // Set left and right edges if width allows
//     if width > 1 {
//         for y in 0..height {
//             map[0][y] = wall_value; // Left edge
//             map[width - 1][y] = wall_value; // Right edge
//         }
//     }

//     // Special case for 1x1 map
//     if width == 1 && height == 1 {
//         map[0][0] = wall_value; // Set the single cell to wall
//     }
// }
