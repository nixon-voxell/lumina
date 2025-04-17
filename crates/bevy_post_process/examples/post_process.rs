use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_post_process::chromatic_aberration::ChromaticAberrationConfig;
use bevy_post_process::vignette::VignetteConfig;
use bevy_post_process::PostProcessPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_plugins(PostProcessPlugin)
        .add_systems(Startup, setup);

    app.run();
}

const X_EXTENT: f32 = 1000.0;
const Y_EXTENT: f32 = 450.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::NONE),
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                near: -500.0,
                far: 500.0,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: 1280.0,
                    max_height: 720.0,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        ChromaticAberrationConfig::default(),
        VignetteConfig {
            intensity: 1.0,
            distance: 0.0,
            // Red tint.
            tint: Vec3::new(1.0, 0.0, 0.0),
        },
    ));

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(180.0 + 180.0 * i as f32 / num_shapes as f32, 0.95, 0.7);

        const Y_COUNT: usize = 8;
        for y in 0..Y_COUNT {
            commands.spawn(MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    f32::lerp(-Y_EXTENT, Y_EXTENT, y as f32 / Y_COUNT as f32),
                    0.0,
                ),
                ..default()
            });
        }
    }
}
