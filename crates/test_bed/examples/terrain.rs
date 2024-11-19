use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_terrain::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((lumina_terrain::TerrainPlugin, lumina_common::CommonPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, generate_terrain)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn generate_terrain(
    mut commands: Commands,
    mut q_camera: Query<&mut Transform, With<Camera2d>>,
    config: TerrainConfig,
    key_input: Res<ButtonInput<KeyCode>>,
    mut generate_evw: EventWriter<GenerateTerrain>,
    mut clear_evw: EventWriter<ClearTerrain>,
    mut terrain_entity: Local<Option<Entity>>,
) {
    if key_input.just_pressed(KeyCode::Space) == false {
        return;
    }

    let entity = match *terrain_entity {
        Some(entity) => entity,
        None => {
            let entity = commands.spawn_empty().id();
            *terrain_entity = Some(entity);

            entity
        }
    };

    if key_input.pressed(KeyCode::ControlLeft) {
        clear_evw.send(ClearTerrain(entity));
        return;
    }

    let Some(config) = config.get() else {
        return;
    };

    let half_size = config.tile_size * 0.5;
    q_camera.single_mut().translation.x = half_size * config.size.x as f32 - half_size;
    q_camera.single_mut().translation.y = half_size * config.size.y as f32 - half_size;

    generate_evw.send(GenerateTerrain {
        seed: rand::random(),
        entity,
        layers: CollisionLayers::ALL,
    });
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
