use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
use lightyear::prelude::*;
use server::*;

use crate::settings::NetworkSettings;
use crate::shared::{shared_config, SERVER_REPLICATION_INTERVAL};

mod lobby;
mod player;
mod ui;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ServerPlugin`.");

        // Lightyear plugins
        let settings = app.world().get_resource::<NetworkSettings>().unwrap();
        app.add_plugins(ServerPlugins::new(server_config(settings)));

        app.add_plugins((ui::ServerUiPlugin, lobby::LobbyPlugin, player::PlayerPlugin))
            .add_systems(Startup, start_server);
    }
}

/// Start the server.
fn start_server(mut commands: Commands) {
    info!("Starting server...");
    commands.start_server();
}

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
