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

pub mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovement>()
            .add_systems(FixedUpdate, player_movement.in_set(FixedSet::Main))
            .add_systems(Update, spawn_bullet_mesh.in_set(FixedSet::Main))
            .add_systems(Update, move_bullets);
    }
}

pub fn shared_handle_player_movement(
    action_state: &ActionState<PlayerAction>,
    player_entity: Entity,
    player_movement_evw: &mut EventWriter<PlayerMovement>,
) {
    if action_state.pressed(&PlayerAction::Move) {
        let Some(movement) = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .map(|axis| axis.xy())
        else {
            return;
        };

        player_movement_evw.send(PlayerMovement {
            movement,
            player_entity,
        });
    }
}

fn player_movement(
    mut q_movements: Query<(&mut LinearVelocity, &mut AngularVelocity, &Rotation), With<PlayerId>>,
    mut player_movement_evr: EventReader<PlayerMovement>,
) {
    const MOVEMENT_SPEED: f32 = 100.0;
    const ROTATION_SPEED: f32 = 0.4;
    const MAX_SPEED: f32 = 400.0;
    for player_movement in player_movement_evr.read() {
        if let Ok((mut linear, mut angular, rotation)) =
            q_movements.get_mut(player_movement.player_entity)
        {
            let movement = player_movement.movement.normalize_or_zero();
            let desired_angle = movement.y.atan2(movement.x);

            angular.0 += rotation.angle_between(Rotation::radians(desired_angle)) * ROTATION_SPEED;

            let direction = Vec2::new(rotation.cos, rotation.sin);

            linear.0 += direction * MOVEMENT_SPEED;

            // Clamp the speed
            linear.0 = linear.clamp_length_max(MAX_SPEED);
        }
    }
}

#[derive(Event)]
pub struct PlayerMovement {
    pub movement: Vec2,
    pub player_entity: Entity,
}

#[derive(Bundle)]
pub struct ReplicatePlayerBundle {
    pub id: PlayerId,
    pub position: Position,
    pub physics_bundle: PhysicsBundle,
}

impl ReplicatePlayerBundle {
    pub fn new(client_id: ClientId, position: Vec2) -> Self {
        Self {
            id: PlayerId(client_id),
            position: Position::new(position),
            physics_bundle: PhysicsBundle::player(),
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
}

impl PhysicsBundle {
    pub fn player() -> Self {
        Self {
            collider: Collider::rectangle(1.0, 1.0),
            rigidbody: RigidBody::Dynamic,
            linear_damping: LinearDamping(2.0),
            angular_damping: AngularDamping(6.0),
        }
    }
}

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
