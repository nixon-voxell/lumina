use bevy::prelude::*;
use client::ComponentSyncMode;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Channels
        app.add_channel::<ReliableChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        // Messages
        app.register_message::<JoinLobby>(ChannelDirection::ClientToServer);
        app.register_message::<ExitLobby>(ChannelDirection::ClientToServer);
        app.register_message::<StartGame>(ChannelDirection::ClientToServer);
        // Components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        app.register_component::<PlayerTranslation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        // Resources
        app.register_resource::<Lobbies>(ChannelDirection::ServerToClient);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Lobby {
    pub players: Vec<ClientId>,
    /// If true, the lobby is in game. If not, it is still in lobby mode.
    pub in_game: bool,
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Lobbies {
    pub lobbies: Vec<Lobby>,
}

impl Lobbies {
    /// Return true if there is an empty lobby available for players to join
    pub fn has_empty_lobby(&self) -> bool {
        if self.lobbies.is_empty() {
            return false;
        }
        self.lobbies.iter().any(|lobby| lobby.players.is_empty())
    }

    /// Remove a client from a lobby
    pub fn remove_client(&mut self, client_id: ClientId) {
        let mut removed_lobby = None;
        for (lobby_id, lobby) in self.lobbies.iter_mut().enumerate() {
            if let Some(index) = lobby.players.iter().position(|id| *id == client_id) {
                lobby.players.remove(index);
                if lobby.players.is_empty() {
                    removed_lobby = Some(lobby_id);
                }
            }
            // if lobby.players.remove(&client_id).is_some() {
            //     if lobby.players.is_empty() {
            //         removed_lobby = Some(lobby_id);
            //     }
            // }
        }
        if let Some(lobby_id) = removed_lobby {
            self.lobbies.remove(lobby_id);
            // always make sure that there is an empty lobby for players to join
            if !self.has_empty_lobby() {
                self.lobbies.push(Lobby::default());
            }
        }
    }
}

#[derive(Channel)]
pub struct ReliableChannel;

// Lobby
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoinLobby {
    pub lobby_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExitLobby {
    pub lobby_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StartGame {
    pub lobby_id: usize,
    pub host: Option<ClientId>,
}

// Player
#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerTranslation(pub Vec2);
