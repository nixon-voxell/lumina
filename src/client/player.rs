use bevy::prelude::*;
use lightyear::prelude::client::Predicted;
// use client::*;
// use leafwing_input_manager::prelude::*;
// use lightyear::prelude::*;

use crate::protocol::player::PlayerTransform;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_transform);
        app.add_systems(Update, follow_player);
    }
}

// /// Player movement
// fn movement(
//     mut q_player_transform: Query<&mut PlayerTransform, With<Predicted>>,
//     action_state: Res<ActionState<PlayerAction>>,
//     time: Res<Time>,
// ) {
//     const SPEED: f32 = 100.0;

//     let Ok(mut player_transform) = q_player_transform.get_single_mut() else {
//         return;
//     };

//     if action_state.pressed(&PlayerAction::Move) {
//         if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
//             player_transform.translation +=
//                 axis_pair.xy().normalize() * time.delta_seconds() * SPEED;
//         }
//     }
// }

/// Apply [`Transform`] from [`PlayerTransform`].
fn apply_transform(
    mut q_transforms: Query<(&mut Transform, &PlayerTransform), Changed<PlayerTransform>>,
) {
    for (mut transform, player_transform) in q_transforms.iter_mut() {
        transform.translation.x = player_transform.translation.x;
        transform.translation.y = player_transform.translation.y;
        transform.rotation = Quat::from_rotation_z(player_transform.rotation);
    }
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    q_player: Query<&PlayerTransform, With<Predicted>>,
    time: Res<Time>,
) {
    // Ensure we have at least one player
    if let Ok(player_transform) = q_player.get_single() {
        for mut camera_transform in q_camera.iter_mut() {
            // Calculate the target position based on player's position
            let target_position = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y, // Adjust this value as needed
                camera_transform.translation.z, // Keep the same z position
            );

            // Smoothly interpolate the camera's position towards the target position
            let lerp_factor = 1.0; // Adjust this value for more or less delay
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, lerp_factor * time.delta_seconds());

            println!("Camera Following Player!");
        }
    } else {
        println!("Player not found!");
    }
}
