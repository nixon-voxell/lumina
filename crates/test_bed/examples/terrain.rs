use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use lumina_shared::terrain::config::{Terrain, TerrainConfigPlugin};
use noisy_bevy::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, generate_terrain)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(TileRef {
        mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        material: materials.add(Color::BLACK),
        collider: Collider::rectangle(1.0, 1.0),
    })
}

fn generate_terrain(
    mut commands: Commands,
    mut q_camera: Query<&mut Transform, With<Camera2d>>,
    terrain: Terrain,
    tile_ref: Res<TileRef>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pool: Local<Vec<Entity>>,
) {
    let Some(terrain) = terrain.config() else {
        return;
    };

    if key_input.just_pressed(KeyCode::Space) == false {
        return;
    }

    for e in pool.drain(..) {
        commands.entity(e).despawn();
    }

    let half_size = terrain.tile_size * 0.5;
    q_camera.single_mut().translation.x = half_size * terrain.size.x as f32 - half_size;
    q_camera.single_mut().translation.y = half_size * terrain.size.y as f32 - half_size;

    let top_seed = rand::random();
    let bottom_seed = rand::random();
    let left_seed = rand::random();
    let right_seed = rand::random();

    for y in 0..terrain.size.y {
        for x in 0..terrain.size.x {
            fn remaped_noise(v: Vec2) -> f32 {
                // Remap from -1.0 -> 1.0 to 0.0 -> 1.0
                // gradient_noise(v) * 0.5 + 0.5
                simplex_noise_2d(v) * 0.5 + 0.5
            }

            let left_dist = u32::abs_diff(x + 1, 0);
            let right_dist = u32::abs_diff(x, terrain.size.x);
            let bottom_dist = u32::abs_diff(y + 1, 0);
            let top_dist = u32::abs_diff(y, terrain.size.y);

            let surr_width = terrain.noise_surr_width as f32;

            // Convert absolute distance to relative distance (may go out of 1.0)
            let gradient = |abs_dist: u32| -> f32 {
                f32::powf(abs_dist as f32 / surr_width, terrain.gradient_pow)
            };

            // Calculate absolute distances to the terrain edges.
            let left_grad = gradient(left_dist);
            let right_grad = gradient(right_dist);
            let bottom_grad = gradient(bottom_dist);
            let top_grad = gradient(top_dist);

            let calc_noise = |scalar: u32, seed: f32, gradient: f32| -> f32 {
                f32::min(
                    1.0,
                    remaped_noise(Vec2::new(terrain.noise_scale * scalar as f32, seed)) * gradient,
                )
            };

            let left_noise = calc_noise(y, left_seed, left_grad);
            let right_noise = calc_noise(y, right_seed, right_grad);
            let bottom_noise = calc_noise(x, bottom_seed, bottom_grad);
            let top_noise = calc_noise(x, top_seed, top_grad);

            // let noise = left_noise * right_noise * bottom_noise * top_noise;
            let noise = left_noise.min(right_noise).min(bottom_noise).min(top_noise);
            let edge_dist = left_dist.min(right_dist).min(bottom_dist).min(top_dist);
            println!("{edge_dist}");

            if noise > terrain.noise_threshold || edge_dist > terrain.noise_surr_width {
                continue;
            }

            let tile = commands
                .spawn(ColorMesh2dBundle {
                    mesh: Mesh2dHandle(tile_ref.mesh.clone()),
                    material: tile_ref.material.clone(),
                    transform: Transform::from_xyz(
                        terrain.tile_size * x as f32,
                        terrain.tile_size * y as f32,
                        0.0,
                    )
                    .with_scale(Vec3::splat(terrain.tile_size)),
                    ..default()
                })
                .id();

            pool.push(tile);
        }
    }
}

#[derive(Resource)]
pub struct TileRef {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub collider: Collider,
}

// fn gradient_noise_dir(mut p: Vec2) -> Vec2 {
//     p %= 289.0;
//     let mut x = (34.0 * p.x + 1.0) * p.x % 289.0 + p.y;
//     x = (34.0 * x + 1.0) * x % 289.0;
//     x = f32::fract(x / 41.0) * 2.0 - 1.0;
//     Vec2::normalize(Vec2::new(x - f32::floor(x + 0.5), f32::abs(x) - 0.5))
// }

// fn gradient_noise(p: Vec2) -> f32 {
//     let ip = Vec2::floor(p);
//     let mut fp = Vec2::fract(p);
//     let d00 = Vec2::dot(gradient_noise_dir(ip), fp);
//     let d01 = Vec2::dot(
//         gradient_noise_dir(ip + Vec2::new(0.0, 1.0)),
//         fp - Vec2::new(0.0, 1.0),
//     );
//     let d10 = Vec2::dot(
//         gradient_noise_dir(ip + Vec2::new(1.0, 0.0)),
//         fp - Vec2::new(1.0, 0.0),
//     );
//     let d11 = Vec2::dot(
//         gradient_noise_dir(ip + Vec2::new(1.0, 1.0)),
//         fp - Vec2::new(1.0, 1.0),
//     );

//     fp = fp * fp * fp * (fp * (fp * 6.0 - 15.0) + 10.0);
//     f32::lerp(f32::lerp(d00, d01, fp.y), f32::lerp(d10, d11, fp.y), fp.x)
// }
