use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::procedural_map::random_walk_cave::{create_cave_map, CaveConfig};

// Constants for default values
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 100;
const TILE_WIDTH: f32 = 100.0;
const TILE_HEIGHT: f32 = 100.0;

// Events
#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct GenerateMapEvent(pub u64);

// Resources
#[derive(Resource)]
pub struct TileConfig {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
    _width: f32,
    _height: f32,
}

#[derive(Resource)]
pub struct SharedRigidBody(RigidBody);

#[derive(Resource)]
pub struct SharedCollider(Collider);

// Cell State Definition
#[derive(Default, Clone, Copy, PartialEq)]
pub enum CellState {
    #[default]
    Filled,
    Empty,
}

// Grid Map Definition
#[derive(Resource, Clone)]
pub struct GridMap {
    states: Vec<CellState>,
    width: u32,
    _height: u32,
    tile_pool: Vec<Entity>,
}

impl GridMap {
    pub fn new(width: u32, height: u32) -> Self {
        let states = vec![CellState::default(); (height * width) as usize];
        Self {
            states,
            width,
            _height: height,
            tile_pool: Vec::new(),
        }
    }

    pub fn states_mut(&mut self) -> &mut Vec<CellState> {
        &mut self.states
    }

    fn get(&self, x: u32, y: u32) -> Option<CellState> {
        if x < self.width && y < self._height {
            Some(self.states[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn is_valid_spawn_point(&self, x: u32, y: u32) -> bool {
        if self.get(x, y) != Some(CellState::Empty) {
            return false;
        }

        let neighbors = [
            self.get(x.wrapping_sub(1), y), // Left
            self.get(x + 1, y),             // Right
            self.get(x, y.wrapping_sub(1)), // Up
            self.get(x, y + 1),             // Down
        ];

        neighbors
            .iter()
            .any(|&cell| cell == Some(CellState::Filled))
    }

    pub fn collect_spawn_points(&self) -> Vec<(u32, u32)> {
        (0..self._height)
            .flat_map(|y| {
                (0..self.width).filter_map(move |x| {
                    if self.get(x, y) == Some(CellState::Empty) && self.is_valid_spawn_point(x, y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

// Setup Functions
pub fn setup_tile_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(TileConfig {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT))),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        _width: TILE_WIDTH,
        _height: TILE_HEIGHT,
    });

    commands.insert_resource(SharedRigidBody(RigidBody::Static));
    commands.insert_resource(SharedCollider(Collider::rectangle(TILE_WIDTH, TILE_HEIGHT)));
}

// Map Generation System
pub fn setup_grid_and_spawn_tiles(
    mut commands: Commands,
    mut generate_map_evr: EventReader<GenerateMapEvent>,
    tile_config: Res<TileConfig>,
    shared_rigid_body: Res<SharedRigidBody>,
    shared_collider: Res<SharedCollider>,
) {
    for generate_map_event in generate_map_evr.read() {
        println!("\n\nGenerate grid with seed: {}", generate_map_event.0);

        let cave_config = CaveConfig {
            map_width: MAP_WIDTH,
            map_height: MAP_HEIGHT,
            random_seed: generate_map_event.0,
            empty_space_percentage: 40.0,
            edge_thickness: 1,
            max_dig_attempts: 10000,
        };

        let mut new_cave_map = GridMap::new(MAP_WIDTH as u32, MAP_HEIGHT as u32);
        let generated_map = create_cave_map(new_cave_map.clone(), cave_config);

        commands.insert_resource(new_cave_map.clone());

        // Precompute neighbor states
        let has_empty_neighbors = precompute_empty_neighbors(&generated_map);

        // Spawn tiles and collect entities needing rigid bodies
        let mut entities_to_add_rigid_body = Vec::new();

        for (i, &state) in generated_map.states.iter().enumerate() {
            if state == CellState::Empty {
                continue; // Skip empty tiles
            }

            let position = Vec2::new(
                (i as u32 % new_cave_map.width) as f32 * tile_config._width,
                (i as u32 / new_cave_map.width) as f32 * tile_config._height,
            );

            let entity_builder = commands.spawn((ColorMesh2dBundle {
                mesh: tile_config.mesh.clone(),
                material: tile_config.material.clone(),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },));

            // Only collect entities that need RigidBody and Collider
            if has_empty_neighbors[i] {
                entities_to_add_rigid_body.push(entity_builder.id());
            }

            // Track spawned tiles
            if new_cave_map.tile_pool.len() <= i {
                new_cave_map.tile_pool.push(entity_builder.id());
            }
        }

        // Add RigidBody and Collider in batch
        for entity_id in entities_to_add_rigid_body {
            commands.entity(entity_id).insert(shared_rigid_body.0);
            commands.entity(entity_id).insert(shared_collider.0.clone());
        }
    }
}

// Neighbor Checking Functions
fn precompute_empty_neighbors(grid_map: &GridMap) -> Vec<bool> {
    grid_map
        .states
        .iter()
        .enumerate()
        .map(|(i, &state)| {
            if state == CellState::Filled {
                check_empty_neighbors(grid_map, i)
            } else {
                false
            }
        })
        .collect()
}

fn check_empty_neighbors(grid_map: &GridMap, index: usize) -> bool {
    let width = grid_map.width as usize;
    let height = grid_map._height as usize;
    let x = index % width;
    let y = index / width;

    let neighbors = [
        (0, 1),  // Down
        (0, -1), // Up
        (1, 0),  // Right
        (-1, 0), // Left
    ];

    for (dx, dy) in &neighbors {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
            let neighbor_index = new_y as usize * width + new_x as usize;
            if grid_map.states[neighbor_index] == CellState::Empty {
                return true; // Found an empty neighbor
            }
        }
    }

    false // No empty neighbors found
}
