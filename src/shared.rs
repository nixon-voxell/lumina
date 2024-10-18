//! This module contains the shared code between the client and the server.
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::*;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub mod action;
pub mod convert_3d_to_2d;
pub mod effector;
pub mod physics;
pub mod player;

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::protocol::ProtocolPlugin,
            crate::ui::UiPlugin,
            convert_3d_to_2d::Convert3dTo2dPlugin,
            player::PlayerPlugin,
            physics::PhysicsPlugin,
            effector::EffectorPlugin,
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

// fn set_owned_entity()

fn set_source_entity<Filter: QueryFilter>(
    mut commands: Commands,
    q_entities: Query<
        Entity,
        (
            Filter,
            Or<(
                // Client
                (With<client::Predicted>, With<client::Interpolated>),
                // Server
                With<server::SyncTarget>,
                // Local
                (
                    Without<Replicated>,
                    Without<client::Predicted>,
                    Without<client::Interpolated>,
                    Without<server::SyncTarget>,
                ),
            )>,
        ),
    >,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(SourceEntity);
    }
}

/// Entity that represents the final source of reference.
/// Source entity follows 3 rules:
///
/// 1. Client source entities will always have [`client::Predicted`] or [`client::Interpolated`]
/// (anything that is not will be controlled by the server through replication - [`Replicated`]).
///
/// 2. Server owned entities will always have [`server::SyncTarget`].
/// (anything that is not is only replicated from the client (extremely rare occasion))
///
/// 3. Locally owned entities must not have any of the above including [`Replicated`].
#[derive(Component, Default)]
pub struct SourceEntity;
