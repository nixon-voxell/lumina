use bevy::prelude::*;

pub mod animator;
pub mod teleporter;

pub mod prelude {
    pub use super::animator::{Animator, Playback, RepeatMode};
    pub use super::teleporter::{
        Teleporter, TeleporterCooldown, TeleporterEffect, TeleporterEnd, TeleporterStart,
    };
    pub use super::GAME_DURATION;
}

// TODO: Read from a config!
// Allow for custom timing.
pub const GAME_DURATION: f32 = 60.0 * 10.0;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((animator::AnimatorPlugin, teleporter::TeleporterPlugin));
    }
}
