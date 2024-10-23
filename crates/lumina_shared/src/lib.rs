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
pub mod procedural_map;
pub mod protocol;
pub mod settings;
pub mod utils;

pub mod prelude {
    pub use crate::action::PlayerAction;
    pub use crate::player::{
        spawn_blueprint_visual, BlueprintType, PlayerId, PlayerInfoType, PlayerInfos,
    };
    pub use crate::protocol::*;
    pub use crate::utils::EntityRoomId;
    pub use crate::{ClientSourceEntity, LocalSourceEntity, ServerSourceEntity, SourceEntity};
}

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            lumina_ui::UiPlugin,
            protocol::ProtocolPlugin,
            convert_3d_to_2d::Convert3dTo2dPlugin,
            player::PlayerPlugin,
            physics::PhysicsPlugin,
            effector::EffectorPlugin,
            procedural_map::GridMapPlugin,
        ));

        app.add_systems(
            PreUpdate,
            (
                (
                    source_hierarchy::<LocalSourceEntity>,
                    source_hierarchy::<ClientSourceEntity>,
                    source_hierarchy::<ServerSourceEntity>,
                ),
                (
                    set_source::<ActionState<PlayerAction>>,
                    set_source::<SpaceShip>,
                    set_source::<Weapon>,
                ),
            )
                .chain()
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

/// Create source entity by inserting [`SourceEntity`] for entities with specific components.
fn set_source<C: Component>(
    mut commands: Commands,
    q_entities: Query<
        Entity,
        (
            With<C>,
            With<PlayerId>,
            Without<SourceEntity>,
            Or<(
                // Local
                With<LocalSourceEntity>,
                // Client
                With<ClientSourceEntity>,
                // Server
                With<ServerSourceEntity>,
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

/// Propagate source component to the children hierarchy.
fn source_hierarchy<Source: Component + Default>(
    mut commands: Commands,
    q_children: Query<
        &Children,
        (
            With<Source>,
            // Just added or the children changes.
            Or<(Added<Source>, Changed<Children>)>,
        ),
    >,
) {
    for children in q_children.iter() {
        for entity in children.iter() {
            commands.entity(*entity).insert(Source::default());
        }
    }
}

/// Entity that represents the final source of reference.
/// Source entity follows 3 rules:
///
/// 1. Client source entities will always have [`ClientSourceEntity`]
///     (anything that is not will be controlled by the server through replication - [`Replicated`]).
///
/// 2. Server owned entities will always have [`ServerSourceEntity`].
///     (anything that is not is only replicated from the client (extremely rare occasion))
///
/// 3. Locally owned entities will always have [`LocalSourceEntity`].
#[derive(Component, Default)]
pub struct SourceEntity;

/// Local source entity needs to be defined manually when spawning entities.
///
/// Any children that follows will also become a local source entity.
#[derive(Component, Default)]
pub struct LocalSourceEntity;

/// Any children that follows will also become a client source entity.
#[derive(Component, Default)]
pub struct ClientSourceEntity;

/// Any children that follows will also become a server source entity.
#[derive(Component, Default)]
pub struct ServerSourceEntity;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SetSourceSet;
