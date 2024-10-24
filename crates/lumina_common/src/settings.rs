//! This module parses the network_settings.ron file and builds a lightyear configuration from it
use std::net::Ipv4Addr;

use bevy::asset::ron;
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings_str = include_str!("../settings.ron");
        let settings = read_settings::<LuminaSettings>(settings_str);
        app.insert_resource(settings);
    }
}

/// We parse the settings.ron file to read the settings
pub fn read_settings<T: DeserializeOwned>(settings_str: &str) -> T {
    ron::de::from_str::<T>(settings_str).expect("Could not deserialize the settings file.")
}

#[derive(Resource, Deserialize, Serialize, Debug, Clone, Copy)]
pub struct LuminaSettings {
    pub fixed_timestep_hz: f64,
    /// In milliseconds.
    pub server_replication_interval: u64,
    pub server: ServerSettings,
    pub client: ClientSettings,
    pub shared: SharedSettings,
}

impl LuminaSettings {
    pub fn server_replication_interval(&self) -> Duration {
        Duration::from_millis(self.server_replication_interval)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ServerSettings {
    /// If true, disable any rendering-related plugins
    pub headless: bool,
    /// If true, enable bevy_inspector_egui
    pub inspector: bool,
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<Conditioner>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct ClientSettings {
    /// If true, enable bevy_inspector_egui
    pub inspector: bool,
    /// The client id
    pub client_id: u64,
    /// The client port to listen on
    pub client_port: u16,
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<Conditioner>,
    pub input_delay_ticks: u16,
    pub correction_ticks_factor: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SharedSettings {
    /// The ip address of the server
    pub server_addr: Ipv4Addr,
    /// The port of the server
    pub server_port: u16,
    /// An id to identify the protocol version
    pub protocol_id: u64,
    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],
    /// compression options
    pub compression: CompressionConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Conditioner {
    /// One way latency in milliseconds
    pub latency_ms: u16,
    /// One way jitter in milliseconds
    pub jitter_ms: u16,
    /// Percentage of packet loss
    pub packet_loss: f32,
}

impl Conditioner {
    pub fn build(&self) -> LinkConditionerConfig {
        LinkConditionerConfig {
            incoming_latency: Duration::from_millis(self.latency_ms as u64),
            incoming_jitter: Duration::from_millis(self.jitter_ms as u64),
            incoming_loss: self.packet_loss,
        }
    }
}
