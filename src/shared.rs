//! This module contains the shared code between the client and the server.
use bevy::{prelude::*, utils::Duration};
use blenvy::BlenvyPlugin;
use lightyear::prelude::*;

pub const FIXED_TIMESTEP_HZ: f64 = 60.0;
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub mod input;
pub mod physics;
pub mod player;

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlenvyPlugin::default());

        app.add_plugins((
            crate::protocol::ProtocolPlugin,
            crate::ui::UiPlugin,
            player::PlayerPlugin,
            physics::PhysicsPlugin,
        ));
    }
}

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode: Mode::Separate,
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MovementSet {
    // Input handling.
    Input,
    // Apply physics.
    Physics,
}
