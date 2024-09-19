use bevy::prelude::*;
use client::ComponentSyncMode;
use lightyear::prelude::*;
use player::{PlayerId, PlayerTransform};
use server::RoomId;

pub mod input;
pub mod player;

pub const PLAYER_REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Channels
        app.add_channel::<ReliableChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        // Messages
        app.register_message::<Matchmake>(ChannelDirection::ClientToServer);
        app.register_message::<ExitLobby>(ChannelDirection::ClientToServer);
        app.register_message::<LobbyStatus>(ChannelDirection::ServerToClient);
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);

        // Components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        app.register_component::<PlayerTransform>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full);

        // Resources
        // app.register_resource::<Lobbies<C>>(ChannelDirection::ServerToClient);

        // Plugins
        app.add_plugins((input::InputPlugin, player::PlayerPlugin));
    }
}

/// Matchmake command (with lobby size encoded) sent from
/// client to server to find an available lobby to join.
#[derive(Serialize, Deserialize, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub struct Matchmake(pub u8);

/// Update on lobby status sent from server to client.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LobbyStatus {
    pub room_id: RoomId,
    pub client_count: u8,
}

/// Exit lobby command sent from client to server when already inside a lobby.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExitLobby;

/// Start game command sent from server to client when the lobby room is full.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StartGame;

/// A simple reliable channel for sending messages through the network reliably.
#[derive(Channel)]
pub struct ReliableChannel;
