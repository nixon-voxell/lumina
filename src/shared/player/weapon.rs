use avian2d::prelude::*;
use bevy::{
    color::palettes::css::WHITE,
    input::mouse::MouseMotion,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::camera::GameCamera;
use crate::shared::input::PlayerAction;
use crate::shared::FixedSet; // Import the GameCamera

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_weapon)
            .add_systems(Update, spawn_bullet_mesh.in_set(FixedSet::Main))
            .add_systems(Update, move_bullets)
            .add_systems(Update, mouse_motion);
    }
}

fn spawn_bullet_mesh(
    time: Res<Time>,
    q_player: Query<&Position, With<client::Predicted>>,
    q_action_states: Query<
        &ActionState<PlayerAction>,
        (With<PrePredicted>, Changed<ActionState<PlayerAction>>),
    >,
    q_camera: Query<&Transform, With<GameCamera>>, // Query the GameCamera transform
    mouse_position: Res<MousePosition>,            // Use the mouse position
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut weapon_query: Query<&mut WeaponProperties>, // Query for WeaponProperties
) {
    let Ok(action_state) = q_action_states.get_single() else {
        return;
    };

    let Ok(player_position) = q_player.get_single() else {
        return;
    };

    let Ok(camera_transform) = q_camera.get_single() else {
        return;
    };

    // Get the weapon properties
    let Ok(mut weapon_properties) = weapon_query.get_single_mut() else {
        return; // Ensure we have valid weapon properties
    };

    // Update the elapsed time since last fire
    weapon_properties.elapsed_time += time.delta_seconds();

    // Check if the attack button is pressed and if enough time has passed since the last fire
    if action_state.pressed(&PlayerAction::UseItem)
        && weapon_properties.elapsed_time >= weapon_properties.firing_rate
    {
        // Calculate direction towards the mouse position adjusted by the camera
        let direction = Vec2::new(
            mouse_position.position.x - (player_position.x + camera_transform.translation.x),
            (player_position.y + camera_transform.translation.y) - mouse_position.position.y, // Corrected y-axis calculation
        )
        .normalize();

        // Adjust bullet spawn position to align with the camera's position
        let spawn_position = Vec3::new(
            player_position.x + camera_transform.translation.x,
            player_position.y + camera_transform.translation.y,
            0.0,
        );

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::from_translation(spawn_position).with_scale(Vec3::splat(16.)),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            BulletMovement {
                direction: Vec3::new(direction.x, direction.y, 0.0),
                speed: 300.0,
            },
            BulletLifetime {
                elapsed_time: 0.0,
                despawn_time: 3.0, // Bullet lives for 3 seconds
            },
        ));

        println!("Bullet Spawned!");

        // Reset the firing timer (set elapsed time back to 0)
        weapon_properties.elapsed_time = 0.0;
    }
}

fn move_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &BulletMovement, &mut BulletLifetime)>,
) {
    for (entity, mut transform, bullet_movement, mut bullet_lifetime) in &mut query {
        // Move the bullet based on its velocity and direction
        transform.translation +=
            bullet_movement.direction * bullet_movement.speed * time.delta_seconds();

        bullet_lifetime.elapsed_time += time.delta_seconds();

        if bullet_lifetime.elapsed_time > bullet_lifetime.despawn_time {
            commands.entity(entity).despawn();
            println!("Bullet Despawned!");
        }
    }
}

fn mouse_motion(
    mut evr_motion: EventReader<MouseMotion>,
    mut mouse_position: ResMut<MousePosition>, // Mutate mouse position
) {
    for ev in evr_motion.read() {
        mouse_position.position += ev.delta; // Update the global mouse position
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

fn setup_weapon(mut commands: Commands) {
    commands.spawn((
        Position::default(), // Ensure you have a Position component or similar
        WeaponProperties {
            firing_rate: 0.3, // Set your desired firing rate here (in seconds)
            magazine_size: 10,
            elapsed_time: 0.0,
        },
    ));

    // Add the resource to track mouse position
    commands.insert_resource(MousePosition {
        position: Vec2::ZERO,
    });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BulletMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
pub struct BulletLifetime {
    pub elapsed_time: f32,
    pub despawn_time: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponProperties {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
    /// Time elapsed since the last bullet was fired
    elapsed_time: f32,
}

#[derive(Resource)]
struct MousePosition {
    pub position: Vec2,
}
