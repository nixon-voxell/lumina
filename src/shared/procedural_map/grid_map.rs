use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

// Constants for default values
const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 100;

pub struct GridMapPlugin;

impl Plugin for GridMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_tile_mesh)
            .add_systems(Update, generate_grid_map)
            .add_systems(PostStartup, test);
    }
}

fn test(mut generate_map_evw: EventWriter<GenerateMap>) {
    generate_map_evw.send(GenerateMap(12314123));
}

fn init_tile_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const WIDTH: f32 = 100.0;
    const HEIGHT: f32 = 100.0;
    commands.insert_resource(TileConifg {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        width: WIDTH,
        height: HEIGHT,
    });
}

fn generate_grid_map(
    mut commands: Commands,
    mut generate_map_evr: EventReader<GenerateMap>,
    mut grid_map: ResMut<GridMap>,
    tile_config: Res<TileConifg>,
) {
    if generate_map_evr.is_empty() {
        return;
    }

    for generate_map in generate_map_evr.read() {
        // Random walk
    }

    let mut tile_pool_index = 0;
    let mut new_tiles = Vec::new();

    for (i, state) in grid_map.states.iter().enumerate() {
        let pos = match state {
            GridState::Empty => continue,
            GridState::Filled => Vec2::new(
                (i as u32 % grid_map.width) as f32 * grid_map.width as f32,
                (i as u32 / grid_map.width) as f32 * grid_map.width as f32,
            ),
        };

        match grid_map.tile_pool.get(tile_pool_index) {
            // Reuse tile pool.
            Some(entity) => {
                commands
                    .entity(*entity)
                    .insert(Transform::from_xyz(pos.x, pos.y, 0.0));
            }
            // If not enough in tile pool, spawn batch.
            None => {
                new_tiles.push(
                    commands
                        .spawn(ColorMesh2dBundle {
                            mesh: tile_config.mesh.clone(),
                            material: tile_config.material.clone(),
                            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                            ..default()
                        })
                        .id(),
                );
            }
        };
        tile_pool_index += 1;
    }

    grid_map.tile_pool.append(&mut new_tiles);
}

#[derive(Default, Clone, Copy)]
pub enum GridState {
    #[default]
    Empty,
    Filled,
}

/// Represents a grid of integers.
#[derive(Resource, Clone)]
pub struct GridMap {
    states: Vec<GridState>,
    width: u32,
    height: u32,
    tile_pool: Vec<Entity>,
}

impl GridMap {
    /// Creates a new grid filled with a specified value (0 or 1).
    pub fn new(width: u32, height: u32) -> Self {
        let states = vec![GridState::default(); (height * width) as usize];
        Self {
            states,
            width,
            height,
            tile_pool: Vec::new(),
        }
    }

    /// Helper method to access grid elements.
    pub fn get(&self, x: usize, y: usize) -> GridState {
        self.states[y * self.width as usize + x]
    }

    /// Helper method to set grid elements.
    pub fn set(&mut self, x: usize, y: usize, value: GridState) {
        self.states[y * self.width as usize + x] = value;
    }
}

// Getter functions
impl GridMap {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(Resource)]
pub struct TileConifg {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
    width: f32,
    height: f32,
}

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct GenerateMap(pub u32);

// Event to trigger the spawning of the rectangle grid
#[derive(Event)]
pub struct SpawnRectangleGridEvent;
