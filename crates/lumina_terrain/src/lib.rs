use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_common::prelude::*;

pub mod config;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((config::TerrainConfigPlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    palette: Res<ColorPalette>,
) {
    commands.insert_resource(TileRef {
        mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        material: materials.add(palette.base1),
        collider: Collider::rectangle(1.0, 1.0),
    })
}

#[derive(Resource)]
pub struct TileRef {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub collider: Collider,
}
