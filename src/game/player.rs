use bevy::prelude::*;

use crate::protocol::{PlayerId, PlayerTranslation};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub id: PlayerId,
    pub player_translation: PlayerTranslation,
    pub sprite_bundle: SpriteBundle,
}
