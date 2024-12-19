use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::BlenvyPlugin;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_common::settings::LuminaSettings;
use lumina_shared::shared_config;
use server::*;

mod game;
mod lobby;
mod player;
mod source_entity;
mod ui;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ServerPlugin`.");

        let settings = app.world().get_resource::<LuminaSettings>().unwrap();
        app.add_plugins((
            ServerPlugins::new(server_config(settings)),
            BlenvyPlugin {
                export_registry: false,
                ..default()
            },
        ))
        .add_plugins((
            ui::ServerUiPlugin,
            source_entity::SourceEntityPlugin,
            lobby::LobbyPlugin,
            player::PlayerPlugin,
            game::GamePlugin,
        ))
        .init_resource::<LobbyInfos>()
        .add_systems(Startup, start_server);
    }
}

/// Start the server.
fn start_server(mut commands: Commands) {
    info!("Starting server...");
    commands.start_server();
}

/// Create the lightyear [`ServerConfig`].
fn server_config(settings: &LuminaSettings) -> ServerConfig {
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
        shared: shared_config(settings),
        // We can specify multiple net configs here, and the server will listen on
        // all of them at the same time. Here we will only use one.
        net: vec![net_config],
        replication: ReplicationConfig {
            // we will send updates to the clients every 100ms
            send_interval: settings.server_replication_interval(),
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
