use bevy::prelude::*;

pub mod teleporter;

pub mod prelude {
    pub use super::teleporter::{
        TeleporterCooldown, TeleporterEffect, TeleporterEnd, TeleporterStart,
    };
}

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(teleporter::TeleporterPlugin);
    }
}
