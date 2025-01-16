use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use bevy_coroutine::prelude::*;
use bevy_motiongfx::MotionGfxPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use blenvy::BlenvyPlugin;
use client::*;
use lightyear::prelude::*;
use lumina_common::settings::LuminaSettings;
use lumina_shared::shared_config;
use lumina_ui::prelude::*;

mod audio;
mod blueprint_visual;
mod camera;
mod effector;
mod in_game;
mod local_lobby;
mod matchmaking;
mod multiplayer_lobby;
mod player;
mod sandbox;
mod source_entity;
mod typ_animation;
mod ui;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ClientPlugin`.");

        let client_id = rand::random();
        let settings = app.world().get_resource::<LuminaSettings>().unwrap();

        app.add_plugins((
            ClientPlugins::new(client_config(client_id, settings)),
            BlenvyPlugin {
                export_registry: cfg!(debug_assertions),
                ..default()
            },
            CoroutinePlugin,
            ShaderUtilsPlugin,
            MotionGfxPlugin,
        ))
        .add_plugins((
            lumina_vfx::VfxPlugin,
            source_entity::SourceEntityPlugin,
            blueprint_visual::BlueprintVisualPlugin,
            audio::AudioPlugin,
            ui::UiPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            effector::EffectorPlugin,
            local_lobby::LocalLobbyPlugin,
            sandbox::SandboxPlugin,
            matchmaking::MatchmakingPlugin,
            multiplayer_lobby::MultiplayerLobbyPlugin,
            in_game::InGamePlugin,
            typ_animation::TypAnimationPlugin::<MainWindowFunc>::default(),
        ))
        .init_state::<Connection>()
        .enable_state_scoped_entities::<Connection>()
        .add_systems(OnEnter(Connection::Connect), connect_server)
        .add_systems(
            PreUpdate,
            (handle_connection, handle_disconnection).after(MainSet::Receive),
        );

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(lumina_dev::log_transition::<Connection>)
            .add_plugins(lumina_dev::log_transition::<ui::Screen>);
    }
}

fn connect_server(mut commands: Commands) {
    commands.connect_client();
}

fn handle_connection(
    mut commands: Commands,
    mut connect_evr: EventReader<ConnectEvent>,
    mut next_connection_state: ResMut<NextState<Connection>>,
) {
    for event in connect_evr.read() {
        let client_id = event.client_id();
        info!("CLIENT: Connected with {client_id:?}");

        next_connection_state.set(Connection::Connected);
        commands.insert_resource(LocalClientId(client_id));
    }
}

fn handle_disconnection(
    mut commands: Commands,
    mut disconnect_evr: EventReader<DisconnectEvent>,
    mut next_connection_state: ResMut<NextState<Connection>>,
) {
    for event in disconnect_evr.read() {
        warn!("Disconnected: {:?}", event.reason);

        next_connection_state.set(Connection::Disconnected);
        commands.remove_resource::<LocalClientId>();
    }
}

/// Create the lightyear [`ClientConfig`].
fn client_config(client_id: u64, settings: &LuminaSettings) -> ClientConfig {
    let server_addr = SocketAddr::new(
        IpAddr::V4(settings.shared.server_addr),
        settings.shared.server_port,
    );

    let auth = Authentication::Manual {
        server_addr,
        client_id,
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
        shared: shared_config(settings),
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

#[derive(Resource, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
struct LocalClientId(pub ClientId);
