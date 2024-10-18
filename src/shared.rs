//! This module contains the shared code between the client and the server.
use action::PlayerAction;
use bevy::prelude::*;
use bevy::utils::Duration;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use player::spaceship::SpaceShip;
use player::weapon::Weapon;
use player::PlayerId;

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

        app.add_systems(
            PostUpdate,
            (
                set_source::<ActionState<PlayerAction>>,
                set_source::<SpaceShip>,
                set_source::<Weapon>,
            )
                .in_set(SetSourceSet),
        );
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

/// Set entity to be a source by inserting [`SourceEntity`].
fn set_source<C: Component>(
    mut commands: Commands,
    q_entities: Query<
        Entity,
        (
            With<C>,
            With<PlayerId>,
            Without<SourceEntity>,
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
        info!(
            "SOURCE: {} with component {}.",
            entity,
            std::any::type_name::<C>()
        );
    }
}

/// Entity that represents the final source of reference.
/// Source entity follows 3 rules:
///
/// 1. Client source entities will always have [`client::Predicted`] or [`client::Interpolated`]
///     (anything that is not will be controlled by the server through replication - [`Replicated`]).
///
/// 2. Server owned entities will always have [`server::SyncTarget`].
///     (anything that is not is only replicated from the client (extremely rare occasion))
///
/// 3. Locally owned entities must not have any of the above including [`Replicated`].
#[derive(Component, Default)]
pub struct SourceEntity;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SetSourceSet;
