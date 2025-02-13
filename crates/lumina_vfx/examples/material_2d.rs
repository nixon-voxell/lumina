use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use lumina_vfx::prelude::*;

pub const CAMERA_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba16Float;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(lumina_vfx::VfxPlugin)
        .add_systems(Startup, ((setup_camera, setup_shapes), setup_vfx).chain())
        .add_systems(Update, (update_image_size, animate_vfx, animate_shapes))
        .run();
}

fn animate_vfx(
    mut q_vfx: Query<&mut HealAbilityMaterial>,
    time: Res<Time>,
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut animation: Local<f32>,
) {
    if kbd_input.just_pressed(KeyCode::Space) {
        *animation = 0.0;
    }
    if let Ok(mut vfx) = q_vfx.get_single_mut() {
        vfx.time = *animation;
    }

    *animation += time.delta_seconds();
}

fn animate_shapes(mut q_shapes: Query<&mut Transform, With<ShapeMarker>>, time: Res<Time>) {
    const SPEED: f32 = 4.0;
    for mut transform in q_shapes.iter_mut() {
        transform.translation.y =
            (time.elapsed_seconds() * SPEED + transform.translation.x).sin() * 50.0;
    }
}

fn setup_camera(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = q_window.single();
    let size = Extent3d {
        width: window.width() as u32,
        height: window.height() as u32,
        ..default()
    };
    println!("{size:?}");

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("main_prepass"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: CAMERA_TEXTURE_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        ..default()
    };
    // Fill image.data with zeroes.
    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: image_handle.clone_weak().into(),
                hdr: true,
                ..default()
            },
            ..default()
        },
        MainPrepassCamera,
        RenderLayers::layer(0),
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        MainCamera,
        RenderLayers::from_layers(&[0, 2]),
    ));

    commands.insert_resource(CameraTexture(image_handle, window.size()));
}

fn setup_vfx(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_texture: Res<CameraTexture>,
) {
    const SIZE: f32 = 400.0;
    let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(SIZE, SIZE)));
    commands.spawn((
        ColorMesh2dBundle {
            mesh: mesh_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        HealAbilityMaterial {
            color: LinearRgba::GREEN,
            time: 0.0,
            screen_texture: camera_texture.0.clone_weak(),
        },
        RenderLayers::layer(2),
    ));
}

fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const X_EXTENT: f32 = 700.0;

    let shapes = [
        Mesh2dHandle(meshes.add(Circle::new(50.0))),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        // Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        // Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
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
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            ShapeMarker,
        ));
    }
}

fn update_image_size(
    q_window: Query<&Window, (Changed<Window>, With<PrimaryWindow>)>,
    mut q_camera: Query<&mut Camera, With<MainPrepassCamera>>,
    mut q_vfx: Query<&mut HealAbilityMaterial>,
    mut camera_texture: ResMut<CameraTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };
    let Ok(mut camera) = q_camera.get_single_mut() else {
        return;
    };
    let Ok(mut vfx) = q_vfx.get_single_mut() else {
        return;
    };

    if camera_texture.1 != window.size() {
        println!("{:?}", window.size());
        if let Some(mut image) = images.remove(&camera_texture.0) {
            let size = Extent3d {
                width: window.width() as u32,
                height: window.height() as u32,
                ..default()
            };

            image.resize(size);

            let image_handle = images.add(image);
            camera.target = image_handle.clone_weak().into();
            vfx.screen_texture = image_handle.clone_weak();
            camera_texture.0 = image_handle;
        }
        camera_texture.1 = window.size()
    }
}

#[derive(Resource)]
pub struct CameraTexture(Handle<Image>, Vec2);

#[derive(Component)]
pub struct MainPrepassCamera;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ShapeMarker;
