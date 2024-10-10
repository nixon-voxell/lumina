use avian2d::prelude::*;
use bevy::{
    color::palettes::css::WHITE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::PlayerAction;
use crate::shared::FixedSet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_weapon)
            .add_systems(Update, spawn_bullet_mesh.in_set(FixedSet::Main))
            .add_systems(Update, move_bullets);
    }
}

fn spawn_bullet_mesh(
    time: Res<Time>,
    mut firing_timer: Local<f32>, // Local variable to keep track of firing time
    q_player: Query<&Position, With<client::Predicted>>,
    q_action_states: Query<
        &ActionState<PlayerAction>,
        (With<PrePredicted>, Changed<ActionState<PlayerAction>>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    weapon_query: Query<&WeaponProperties>, // Query for WeaponProperties
) {
    // Update the firing timer
    *firing_timer += time.delta_seconds();

    let Ok(action_state) = q_action_states.get_single() else {
        return;
    };

    let Ok(player_position) = q_player.get_single() else {
        return;
    };

    // Get the weapon properties
    let Ok(weapon_properties) = weapon_query.get_single() else {
        return; // Ensure we have valid weapon properties
    };

    // Check if the attack button is pressed and if enough time has passed
    if action_state.pressed(&PlayerAction::Attack) && *firing_timer >= weapon_properties.firing_rate
    {
        let initial_pos = Vec3::ZERO;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::from_xyz(player_position.x, player_position.y, 0.0)
                    .with_scale(Vec3::splat(16.)),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
            BulletMovement {
                direction: Vec3::new(1.0, 0.0, 0.0),
                speed: 300.0,
            },
            DistanceTraveled {
                start_pos: initial_pos,
                max_distance: 700.0,
            },
        ));

        println!("Circle Spawned!");

        // Reset the firing timer
        *firing_timer = 0.0;
    }
}

fn move_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &BulletMovement, &DistanceTraveled)>,
) {
    for (entity, mut transform, bullet_movement, distance_traveled) in &mut query {
        // Move the circle based on its velocity
        transform.translation +=
            bullet_movement.direction * bullet_movement.speed * time.delta_seconds();

        // Calculate the distance traveled from the start position
        let distance = transform.translation.distance(distance_traveled.start_pos);

        // Despawn the circle if it has traveled beyond the maximum distance
        if distance > distance_traveled.max_distance {
            commands.entity(entity).despawn();
            println!("Circle Despawned after reaching max distance!");
        }
    }
}

fn setup_weapon(mut commands: Commands) {
    commands.spawn((
        Position::default(), // Ensure you have a Position component or similar
        WeaponProperties {
            firing_rate: 0.3, // Set your desired firing rate here (in seconds)
            magazine_size: 10,
        },
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BulletMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
pub struct DistanceTraveled {
    start_pos: Vec3,
    max_distance: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponProperties {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
}

/// The current number of bullets left in the turret.
#[derive(Component)]
pub struct WeaponMagazine(pub u32);
