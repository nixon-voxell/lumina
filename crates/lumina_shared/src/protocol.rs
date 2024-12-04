use avian2d::prelude::*;
use bevy::prelude::*;
use client::ComponentSyncMode;
use lightyear::prelude::*;
use lightyear::utils::avian2d::*;
use server::RoomId;
use strum::EnumCount;

use crate::action::PlayerAction;
use crate::blueprints::{SpaceshipType, WeaponType};
use crate::health::{Health, MaxHealth};
use crate::player::prelude::*;

pub const INPUT_REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

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
        app.register_message::<LobbyUpdate>(ChannelDirection::ServerToClient);
        app.register_message::<LobbyData>(ChannelDirection::ServerToClient);
        app.register_message::<StartGame>(ChannelDirection::ServerToClient);
        app.register_message::<EndGame>(ChannelDirection::ServerToClient);
        app.register_message::<GameScore>(ChannelDirection::ServerToClient);

        // Input
        app.add_plugins(LeafwingInputPlugin::<PlayerAction>::default());

        // Components
        // Health
        app.register_component::<MaxHealth>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Simple)
            .add_interpolation(client::ComponentSyncMode::Simple);

        app.register_component::<Health>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Simple)
            .add_interpolation(client::ComponentSyncMode::Simple);

        // Player
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<TeamType>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<Spaceship>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<SpaceshipType>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<Weapon>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<WeaponType>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        // Physics
        app.register_component::<RigidBody>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Once)
            .add_interpolation(client::ComponentSyncMode::Once);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        app.register_component::<LinearDamping>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Full);

        app.register_component::<AngularDamping>(ChannelDirection::ServerToClient)
            .add_prediction(client::ComponentSyncMode::Full);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(linear_velocity::lerp)
            .add_correction_fn(linear_velocity::lerp);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(angular_velocity::lerp)
            .add_correction_fn(angular_velocity::lerp);
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameScore {
    pub scores: [u8; TeamType::COUNT],
    pub max_score: u8,
}

impl GameScore {
    pub fn new(max_score: u8) -> Self {
        Self {
            scores: [0; TeamType::COUNT],
            max_score,
        }
    }
}

/// Matchmake command (with lobby size encoded) sent from
/// client to server to find an available lobby to join.
#[derive(Serialize, Deserialize, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub struct Matchmake(pub u8);

/// Update on lobby status sent from server to client.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LobbyUpdate {
    pub client_count: u8,
}

/// Data required from the clients when they joined a lobby.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LobbyData {
    pub room_id: RoomId,
}

/// Exit lobby command sent from client to server when already inside a lobby.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ExitLobby;

/// Start game command sent from server to client when the lobby room is full.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StartGame {
    pub seed: u32,
}

/// End game command sent from server to client either when 1 team wins or timer runs out.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct EndGame;

/// A simple reliable channel for sending messages through the network reliably.
#[derive(Channel)]
pub struct ReliableChannel;
