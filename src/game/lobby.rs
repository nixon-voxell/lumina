use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use steamworks::LobbyId;

use crate::steam::Client;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LobbyState>()
            .init_resource::<SteamLobby>()
            .add_systems(OnEnter(LobbyState::CreateLobby), create_steam_lobby)
            .add_systems(OnEnter(LobbyState::LeaveLobby), leave_steam_lobby)
            .add_systems(
                Update,
                steam_lobby_read.run_if(in_state(LobbyState::CreateLobby)),
            );
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LobbyState {
    #[default]
    Default,
    CreateLobby,
    LobbyCreated,
    LeaveLobby,
}

#[derive(Resource, Debug, Clone)]
pub struct SteamLobby {
    id: Arc<Mutex<Option<LobbyId>>>,
    lobby_type: steamworks::LobbyType,
    player_count: u32,
}

impl Default for SteamLobby {
    fn default() -> Self {
        Self {
            id: Arc::default(),
            lobby_type: steamworks::LobbyType::Public,
            player_count: 2,
        }
    }
}

impl SteamLobby {
    pub fn get_id(&self) -> Arc<Mutex<Option<LobbyId>>> {
        self.id.clone()
    }

    pub fn read_id(&self) -> Option<LobbyId> {
        *self.id.lock().unwrap()
    }
}

fn create_steam_lobby(client: Res<Client>, lobby: Res<SteamLobby>) {
    let matchmaking = client.matchmaking();

    let lobby_id = lobby.get_id();
    matchmaking.create_lobby(lobby.lobby_type, lobby.player_count, {
        move |id| {
            match id {
                Ok(id) => {
                    *lobby_id.lock().unwrap() = Some(id);
                    info!("Created lobby: {id:?}");
                }
                Err(e) => error!("Unable to create lobby: {e:?}"),
            };
        }
    });
}

fn leave_steam_lobby(
    client: Res<Client>,
    lobby: Res<SteamLobby>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    let matchmaking = client.matchmaking();

    if let Some(id) = lobby.read_id() {
        matchmaking.leave_lobby(id);
        info!("Left lobby: {id:?}");
    } else {
        error!("There is no lobby to leave!");
    }

    next_lobby_state.set(LobbyState::Default);
}

fn steam_lobby_read(lobby: Res<SteamLobby>, mut next_lobby_state: ResMut<NextState<LobbyState>>) {
    println!("{:?}", lobby.read_id());

    if lobby.read_id().is_some() {
        next_lobby_state.set(LobbyState::LobbyCreated);
    }
}
