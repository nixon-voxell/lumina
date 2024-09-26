use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::settings::NetworkSettings;
use crate::shared::shared_config;
use bevy::{prelude::*, render::view::RenderLayers};

use client::*;
use lightyear::prelude::*;

mod lobby;
mod player;
mod ui;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ClientPlugin`.");

        // Lightyear plugins
        let settings = app.world().get_resource::<NetworkSettings>().unwrap();
        app.add_plugins(ClientPlugins::new(client_config(settings)));

        app.add_plugins((lobby::LobbyPlugin, ui::UiPlugin, player::PlayerPlugin))
            .init_state::<Connection>()
            .enable_state_scoped_entities::<Connection>()
            .add_systems(Startup, spawn_game_camera)
            .add_systems(OnEnter(Connection::Connect), connect_server)
            .add_systems(
                PreUpdate,
                (handle_connection, handle_disconnection).after(MainSet::Receive),
            );

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev_tools::log_transition::<Connection>)
            .add_plugins(crate::dev_tools::log_transition::<lobby::LobbyState>);
    }
}

fn connect_server(mut commands: Commands) {
    commands.connect_client();
}

fn handle_connection(
    mut commands: Commands,
    mut connect_evr: EventReader<ConnectEvent>,
    mut connection: ResMut<NextState<Connection>>,
) {
    for event in connect_evr.read() {
        let client_id = event.client_id();
        info!("Connected with Id: {client_id:?}");

        connection.set(Connection::Connected);
        commands.insert_resource(MyClientId(client_id));
    }
}

fn handle_disconnection(
    mut disconnect_evr: EventReader<DisconnectEvent>,
    mut connection: ResMut<NextState<Connection>>,
) {
    for event in disconnect_evr.read() {
        warn!("Disconnected: {:?}", event.reason);

        connection.set(Connection::Disconnected);
    }
}

/// Spawn camera for game rendering (render layer 0).
fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::NONE.into(),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(0),
    ));
}

/// Create the lightyear [`ClientConfig`].
fn client_config(settings: &NetworkSettings) -> ClientConfig {
    let server_addr = SocketAddr::new(
        IpAddr::V4(settings.shared.server_addr),
        settings.shared.server_port,
    );

    let auth = Authentication::Manual {
        server_addr,
        client_id: rand::random(),
        private_key: settings.shared.private_key,
        protocol_id: settings.shared.protocol_id,
    };

    let transport = ClientTransport::UdpSocket(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        settings.client.client_port,
    ));
    let conditioner = settings.client.conditioner.map(|c| c.build());

    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        transport,
        conditioner,
        compression: settings.shared.compression,
    };

    // The NetConfig specifies how we establish a connection with the server.
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    ClientConfig {
        shared: shared_config(),
        net: net_config,
        prediction: PredictionConfig {
            minimum_input_delay_ticks: settings.client.input_delay_ticks,
            correction_ticks_factor: settings.client.correction_ticks_factor,
            ..default()
        },
        ..default()
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Connection {
    #[default]
    Connect,
    Connected,
    Disconnected,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
struct MyClientId(pub ClientId);
