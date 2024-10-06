use avian2d::prelude::*;
use bevy::prelude::*;

use super::player::MyPlayer;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_camera)
            .add_systems(Update, follow_player);
    }
}

/// Spawn camera for game rendering (default to render layer 0).
fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        GameCamera,
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::Srgba(Srgba::hex("19181A").unwrap()).into(),
                ..default()
            },
            ..default()
        },
    ));
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    q_player: Query<&Position, With<MyPlayer>>,
    time: Res<Time>,
) {
    // Adjust this value for more or less delay.
    const LERP_FACTOR: f32 = 2.0;

    // Ensure we have at least one player.
    let Ok(player_pos) = q_player.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };

    // Calculate the target position based on player's position.
    let target_position = Vec3::new(
        player_pos.x,
        player_pos.y,
        camera_transform.translation.z, // Keep the same z position
    );

    // Smoothly interpolate the camera's position towards the target position.
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        // Clamp within 1.0 to prevent overshooting
        f32::min(1.0, LERP_FACTOR * time.delta_seconds()),
    );
}

#[derive(Component)]
pub(super) struct GameCamera;
