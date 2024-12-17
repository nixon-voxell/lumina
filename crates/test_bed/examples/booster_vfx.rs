use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ShaderUtilsPlugin,
            Material2dPlugin::<BoosterMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct BoosterMaterial {
    #[uniform(0)]
    primary_color: LinearRgba,
    #[uniform(1)]
    secondary_color: LinearRgba,
    #[uniform(2)]
    rotation: f32,
    #[uniform(3)]
    inv_scale: f32,
}

impl Material2d for BoosterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vfx/booster.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BoosterMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
        material: materials.add(BoosterMaterial {
            primary_color: LinearRgba::rgb(0.0, 2.0, 4.0),
            secondary_color: LinearRgba::rgb(0.0, 0.0, 2.0),
            rotation: 0.0,
            inv_scale: 1.0,
        }),
        transform: Transform::from_scale(Vec3::splat(300.0)),
        ..Default::default()
    });
}

fn update(
    q_booster: Query<&Handle<BoosterMaterial>>,
    mut materials: ResMut<Assets<BoosterMaterial>>,
    time: Res<Time>,
) {
    let Some(booster) = materials.get_mut(q_booster.single()) else {
        return;
    };

    booster.rotation = f32::sin(time.elapsed_seconds());
    booster.inv_scale = f32::cos(time.elapsed_seconds()).mul_add(0.5, 1.5);
}
