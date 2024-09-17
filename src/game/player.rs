use bevy::prelude::*;
use client::{ComponentSyncMode, Predicted};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::{input::PlayerAction, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(GameState::InGame)));

        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        app.register_component::<PlayerTransform>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
    }
}

/// Player movement
fn movement(
    mut q_player_transform: Query<&mut PlayerTransform, With<Predicted>>,
    action_state: Res<ActionState<PlayerAction>>,
) {
    let Ok(mut player_transform) = q_player_transform.get_single_mut() else {
        return;
    };

    if action_state.pressed(&PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
            println!("Move: ({}, {})", axis_pair.x(), axis_pair.y());
            player_transform.translation += axis_pair.xy();
        }
    }
}

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

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub id: PlayerId,
    pub player_transform: PlayerTransform,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerTransform {
    pub translation: Vec2,
    pub rotation: f32,
}
