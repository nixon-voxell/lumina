use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::procedural_map::random_walk_cave::{create_cave_map, CaveConfig};

// Constants for default values
pub const DEFAULT_WIDTH: usize = 100;
pub const DEFAULT_HEIGHT: usize = 100;

/// This event is needed to start making a new map.
/// The number is used to make sure the map can be the same every time if we want.
#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct GenerateMapEvent(pub u64);

// Tile configuration
#[derive(Resource)]
pub struct TileConfig {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
    _width: f32,
    _height: f32,
}

/// Shows if a cell in the grid is empty or filled.
/// It is needed to know where we can place things or dig paths.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum CellState {
    #[default]
    Filled, // The cell is filled
    Empty, // The cell is empty
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

/// This function sets up the tile mesh and material needed to draw the grid.
/// It helps performance by creating these resources once at the start, so we don't
/// have to make new ones every time we draw a tile.
pub fn initialize_tile_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const TILE_WIDTH: f32 = 100.0;
    const TILE_HEIGHT: f32 = 100.0;
    commands.insert_resource(TileConfig {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT))),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        _width: TILE_WIDTH,
        _height: TILE_HEIGHT,
    });
}

pub fn setup_grid_and_spawn_tiles(
    mut commands: Commands,
    mut generate_map_event_reader: EventReader<GenerateMapEvent>,
    tile_config: Res<TileConfig>,
) {
    for generate_map_event in generate_map_event_reader.read() {
        // Create a CaveConfig instance
        let cave_config = CaveConfig {
            map_width: DEFAULT_WIDTH,
            map_height: DEFAULT_HEIGHT,
            random_seed: generate_map_event.0,
            empty_space_percentage: 40.0,
            edge_thickness: 1,
            max_dig_attempts: 10000,
        };

        // Generate the cave map
        let mut new_cave_map = create_cave_map(
            GridMap::new(DEFAULT_WIDTH as u32, DEFAULT_HEIGHT as u32),
            cave_config,
        );

        // Insert the new GridMap resource
        commands.insert_resource(new_cave_map.clone());

        // Spawn tiles in filled cells
        let mut tile_pool_index = 0;
        let mut new_tiles = Vec::new();
        let mut filled_count = 0;
        let mut empty_count = 0;

        for (i, state) in new_cave_map.states.iter().enumerate() {
            let position = match state {
                CellState::Empty => {
                    empty_count += 1;
                    continue;
                }
                CellState::Filled => {
                    filled_count += 1;
                    Vec2::new(
                        (i as u32 % new_cave_map.width) as f32 * new_cave_map.width as f32,
                        (i as u32 / new_cave_map.width) as f32 * new_cave_map.width as f32,
                    )
                }
            };

            match new_cave_map.tile_pool.get(tile_pool_index) {
                // Reuse tile pool.
                Some(entity) => {
                    commands
                        .entity(*entity)
                        .insert(Transform::from_xyz(position.x, position.y, 0.0));
                }
                // If not enough in tile pool, spawn batch.
                None => {
                    new_tiles.push(
                        commands
                            .spawn(ColorMesh2dBundle {
                                mesh: tile_config.mesh.clone(),
                                material: tile_config.material.clone(),
                                transform: Transform::from_xyz(position.x, position.y, 0.0),
                                ..default()
                            })
                            .id(),
                    );
                }
            };
            tile_pool_index += 1;
        }

        new_cave_map.tile_pool.append(&mut new_tiles);

        // Debug output for filled and empty cells
        info!("Filled cells: {}", filled_count);
        info!("Empty cells: {}", empty_count);
    }
}
