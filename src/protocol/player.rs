use std::ops::{Add, Mul};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::input::PlayerAction;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);
        app.register_component::<PlayerTransform>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Full)
            .add_interpolation(client::ComponentSyncMode::Full)
            .add_linear_interpolation_fn();
    }
}

pub fn shared_movement_behaviour(
    mut transform: Mut<PlayerTransform>,
    action_state: &ActionState<PlayerAction>,
) {
    const SPEED: f32 = 8.0;

    if action_state.pressed(&PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
            println!("Move: ({}, {})", axis_pair.x(), axis_pair.y());
            transform.translation += axis_pair.xy() * SPEED;
        }
    }

    if action_state.pressed(&PlayerAction::Interact) {
        println!("Interact");
    }

    if action_state.pressed(&PlayerAction::UseItem) {
        println!("UseItem");
    }
}

#[derive(Bundle)]
pub struct PlayerReplicateBundle {
    pub id: PlayerId,
    pub player_transform: PlayerTransform,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerTransform {
    pub translation: Vec2,
    pub rotation: f32,
}

impl Add for PlayerTransform {
    type Output = PlayerTransform;
    #[inline]
    fn add(self, rhs: PlayerTransform) -> PlayerTransform {
        PlayerTransform {
            translation: self.translation.add(rhs.translation),
            rotation: self.rotation + rhs.rotation,
        }
    }
}

impl Mul<f32> for &PlayerTransform {
    type Output = PlayerTransform;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerTransform {
            translation: self.translation * rhs,
            rotation: self.rotation * rhs,
        }
    }
}
