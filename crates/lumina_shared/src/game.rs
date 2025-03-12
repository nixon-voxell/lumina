use bevy::prelude::*;

pub mod animator;
pub mod teleporter;

pub mod prelude {
    pub use super::animator::{Animator, Playback, RepeatMode};
    pub use super::teleporter::{
        Teleporter, TeleporterCooldown, TeleporterEffect, TeleporterEnd, TeleporterStart,
    };
}

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((animator::AnimatorPlugin, teleporter::TeleporterPlugin));
    }
}
