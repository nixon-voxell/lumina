use bevy::prelude::*;
// use client::*;
// use leafwing_input_manager::prelude::*;
// use lightyear::prelude::*;

use crate::protocol::player::PlayerTransform;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_transform);
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
