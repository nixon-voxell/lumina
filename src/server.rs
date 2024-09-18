use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use lightyear::prelude::*;
use server::*;

use crate::shared::{shared_config, SERVER_REPLICATION_INTERVAL};

mod lobby;
mod ui;

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ServerPlugin`.");
        // Lightyear plugins
        app.add_plugins(ServerPlugins::new(server_config()));

        app.add_plugins((lobby::LobbyPlugin, ui::ServerUiPlugin))
            .add_systems(Startup, start_server);
    }
}

/// Start the server.
fn start_server(mut commands: Commands) {
    info!("Starting server...");
    commands.start_server();
}

/// Create the lightyear [`ServerConfig`].
fn server_config() -> ServerConfig {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    ServerConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        // we can specify multiple net configs here, and the server will listen on all of them
        // at the same time. Here we will only use one
        net: vec![net_config],
        replication: ReplicationConfig {
            // we will send updates to the clients every 100ms
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    }
}
