use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::procedural_map::random_walk_cave::{create_cave_map, CaveConfig};

// Constants for default values
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 100;
const TILE_WIDTH: f32 = 100.0;
const TILE_HEIGHT: f32 = 100.0;

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct GenerateMapEvent(pub u64);

#[derive(Resource)]
pub struct TileConfig {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
    _width: f32,
    _height: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CellState {
    #[default]
    Filled,
    Empty,
}

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

    pub fn _get(&self, x: usize, y: usize) -> CellState {
        self.states[y * self.width as usize + x]
    }

    pub fn _set(&mut self, x: usize, y: usize, value: CellState) {
        self.states[y * self.width as usize + x] = value;
    }

    pub fn _width(&self) -> u32 {
        self.width
    }

    pub fn _height(&self) -> u32 {
        self._height
    }
}

#[derive(Resource)]
pub struct SharedRigidBody(RigidBody);

#[derive(Resource)]
pub struct SharedCollider(Collider);

pub fn initialize_tile_mesh(
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
}

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(SharedRigidBody(RigidBody::Static));
    commands.insert_resource(SharedCollider(Collider::rectangle(TILE_WIDTH, TILE_HEIGHT)));
}

pub fn setup_grid_and_spawn_tiles(
    mut commands: Commands,
    mut generate_map_event_reader: EventReader<GenerateMapEvent>,
    tile_config: Res<TileConfig>,
    shared_rigid_body: Res<SharedRigidBody>,
    shared_collider: Res<SharedCollider>,
) {
    for generate_map_event in generate_map_event_reader.read() {
        let cave_config = CaveConfig {
            map_width: MAP_WIDTH,
            map_height: MAP_HEIGHT,
            random_seed: generate_map_event.0,
            empty_space_percentage: 40.0,
            edge_thickness: 1,
            max_dig_attempts: 10000,
        };

        // Create a new GridMap
        let mut new_cave_map = GridMap::new(MAP_WIDTH as u32, MAP_HEIGHT as u32);
        let generated_map = create_cave_map(new_cave_map.clone(), cave_config);

        // Insert the GridMap resource after creating the cave map
        commands.insert_resource(new_cave_map.clone());

        // Precompute neighbor states
        let mut has_empty_neighbors = vec![false; generated_map.states.len()];

        for (i, &state) in generated_map.states.iter().enumerate() {
            if state == CellState::Filled {
                has_empty_neighbors[i] = check_empty_neighbors(&generated_map, i);
            }
        }

        // Spawn tiles using the generated cave map
        for (i, &state) in generated_map.states.iter().enumerate() {
            if state == CellState::Empty {
                continue; // Skip empty tiles
            }

            let position = Vec2::new(
                (i as u32 % new_cave_map.width) as f32 * tile_config._width,
                (i as u32 / new_cave_map.width) as f32 * tile_config._height,
            );

            // Create the entity with the ColorMesh2dBundle
            let mut entity_builder = commands.spawn((ColorMesh2dBundle {
                mesh: tile_config.mesh.clone(),
                material: tile_config.material.clone(),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },));

            // Only add RigidBody and Collider if there are empty neighbors
            if has_empty_neighbors[i] {
                entity_builder.insert(shared_rigid_body.0.clone()); // Add RigidBody
                entity_builder.insert(shared_collider.0.clone()); // Add Collider
            }

            // Get the entity ID
            let tile_entity = entity_builder.id();

            // Track spawned tiles
            if new_cave_map.tile_pool.len() <= i {
                new_cave_map.tile_pool.push(tile_entity);
            }
        }
    }
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
