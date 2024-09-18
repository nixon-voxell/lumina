use std::ops::{Add, Mul};

use bevy::prelude::*;
use client::{ComponentSyncMode, Predicted};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::{input::PlayerAction, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        app.register_component::<PlayerTransform>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();
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
