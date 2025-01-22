//! This module contains the shared code between the client and the server.

use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::*;
use lumina_common::settings::LuminaSettings;

pub mod action;
pub mod blueprints;
pub mod effector;
pub mod health;
pub mod player;
pub mod protocol;

mod type_registry;

pub mod prelude {
    pub use crate::action::PlayerAction;
    pub use crate::blueprints::*;
    pub use crate::effector::{MatchmakeEffector, TesseractEffector};
    pub use crate::health::{Health, MaxHealth};
    pub use crate::player::prelude::*;
    pub use crate::protocol::*;
}

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_plugins((
                type_registry::TypeRegistryPlugin,
                lumina_ui::UiPlugin,
                lumina_terrain::TerrainPlugin,
                protocol::ProtocolPlugin,
                player::PlayerPlugin,
                effector::EffectorPlugin,
                blueprints::BlueprintsPlugin,
                health::HealthPlugin,
            ));
    }
}

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config(settings: &LuminaSettings) -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: settings.server_replication_interval(),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / settings.fixed_timestep_hz),
        },
        mode: Mode::Separate,
    }
}
