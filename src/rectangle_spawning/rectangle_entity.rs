use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// Represents the dimensions of a rectangle
#[derive(Debug, Clone, Copy, Component)]
pub struct RectangleDimension(f32);

impl RectangleDimension {
    pub fn new(value: f32) -> Option<Self> {
        if value > 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Default for RectangleDimension {
    fn default() -> Self {
        Self(1.0)
    }
}

// Configuration for the rectangle's properties
#[derive(Debug, Default, Clone, Component)]
pub struct RectangleConfig {
    pub width: RectangleDimension,
    pub height: RectangleDimension,
    pub color: Color,
}

impl RectangleConfig {
    pub fn new(width: RectangleDimension, height: RectangleDimension, color: Color) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    pub fn default() -> Self {
        Self::new(
            RectangleDimension::new(100.0).unwrap(),
            RectangleDimension::new(100.0).unwrap(),
            Color::srgb(0.0, 0.5, 0.8),
        )
    }
}

// Creates a mesh for the rectangle
fn create_rectangle_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
) -> Result<Mesh2dHandle, String> {
    let rectangle_mesh = Rectangle::new(width, height);
    let handle = meshes.add(rectangle_mesh);
    Ok(Mesh2dHandle(handle))
}

// Spawns a rectangle entity with the specified configuration and position
pub fn spawn_rectangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    config: RectangleConfig,
    position: Transform,
) -> Result<(), String> {
    let rectangle_handle =
        create_rectangle_mesh(meshes, config.width.value(), config.height.value())?;
    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle_handle,
        material: materials.add(config.color),
        transform: position,
        ..default()
    });
    Ok(())
}
