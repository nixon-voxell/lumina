use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use lightyear::prelude::*;
use server::*;

use crate::settings::NetworkSettings;
use crate::shared::{shared_config, SERVER_REPLICATION_INTERVAL};
use crate::utils::EntityRoomId;

mod lobby;
mod player;
mod ui;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ServerPlugin`.");

        // Lightyear plugins
        let settings = app.world().get_resource::<NetworkSettings>().unwrap();
        app.add_plugins((
            ServerPlugins::new(server_config(settings)),
            BlenvyPlugin {
                export_registry: false,
                ..default()
            },
        ));

        app.add_plugins((ui::ServerUiPlugin, lobby::LobbyPlugin, player::PlayerPlugin))
            .init_resource::<LobbyInfos>()
            .add_systems(Startup, start_server)
            .add_systems(
                PreUpdate,
                (server_source, server_source_hierarchy).before(MainSet::Send),
            );
    }
}

/// Start the server.
fn start_server(mut commands: Commands) {
    info!("Starting server...");
    commands.start_server();
}

/// Insert [`ServerSourceEntity`] to newly added [`SyncTarget`] entities.
fn server_source(mut commands: Commands, q_entities: Query<Entity, Added<SyncTarget>>) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(ServerSourceEntity);
    }
}

/// Propagate [`ServerSourceEntity`] to the children hierarchy.
fn server_source_hierarchy(
    mut commands: Commands,
    q_children: Query<
        &Children,
        (
            With<ServerSourceEntity>,
            // Just added or the children changes.
            Or<(Added<ServerSourceEntity>, Changed<Children>)>,
        ),
    >,
) {
    for children in q_children.iter() {
        for entity in children.iter() {
            commands.entity(*entity).insert(ServerSourceEntity);
        }
    }
}

/// Any entity with [`SyncTarget`] is a server source entity.
///
/// Any children that follows that will also become a server source entity.
#[derive(Component, Default)]
pub struct ServerSourceEntity;

/// Create the lightyear [`ServerConfig`].
fn server_config(settings: &NetworkSettings) -> ServerConfig {
    let transport = ServerTransport::UdpSocket(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::UNSPECIFIED,
        settings.shared.server_port,
    )));
    let conditioner = settings.server.conditioner.map(|c| c.build());

    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        transport,
        conditioner,
        compression: settings.shared.compression,
    };

    // The NetConfig specifies how we establish a connection with the server.
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default()
            .with_key(settings.shared.private_key)
            .with_protocol_id(settings.shared.protocol_id),
    };
    ServerConfig {
        shared: shared_config(),
        // We can specify multiple net configs here, and the server will listen on
        // all of them at the same time. Here we will only use one.
        net: vec![net_config],
        replication: ReplicationConfig {
            // we will send updates to the clients every 100ms
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct LobbyInfos(HashMap<ClientId, Entity>);

impl LobbyInfos {
    pub fn get_room_id(&self, client_id: &ClientId) -> Option<RoomId> {
        self.get(client_id).map(|e| e.room_id())
    }
}
