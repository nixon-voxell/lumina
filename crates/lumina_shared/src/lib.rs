//! This module contains the shared code between the client and the server.
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::*;
use lumina_common::settings::LuminaSettings;

pub mod action;
pub mod effector;
pub mod player;
pub mod procedural_map;
pub mod protocol;

mod type_registry;

pub mod prelude {
    pub use crate::action::PlayerAction;
    pub use crate::player::{PlayerId, PlayerInfoType, PlayerInfos};
    pub use crate::protocol::*;
}

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            type_registry::TypeRegistryPlugin,
            lumina_ui::UiPlugin,
            protocol::ProtocolPlugin,
            player::PlayerPlugin,
            effector::EffectorPlugin,
            procedural_map::GridMapPlugin,
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
