use avian2d::prelude::*;
use bevy::{
    color::palettes::css::WHITE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::player;

use super::{input::PlayerAction, FixedSet};

#[derive(Component)]
pub struct BulletMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]

pub struct DistanceTraveled {
    start_pos: Vec3,
    max_distance: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullet_mesh.in_set(FixedSet::Main))
            .add_systems(Update, move_bullets);
    }
}

fn spawn_bullet_mesh(
    //keyboard_input: Res<ButtonInput<KeyCode>>,
    q_player: Query<&Position, With<client::Predicted>>,
    q_action_states: Query<
        &ActionState<PlayerAction>,
        (With<PrePredicted>, Changed<ActionState<PlayerAction>>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(action_state) = q_action_states.get_single() else {
        return;
    };

    let Ok(player_position) = q_player.get_single() else {
        return;
    };
    //Make sure to spawn from Player
    if action_state.pressed(&PlayerAction::Attack) {
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
