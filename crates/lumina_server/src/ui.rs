use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use super::lobby::Lobby;

pub(super) struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyListUi>()
            .compile_typst_func::<LobbyListUi, LobbyListFunc>()
            .init_resource::<LobbyListFunc>()
            .add_systems(Update, (lobbies, push_to_main_window::<LobbyListFunc>()));
    }
}

fn lobbies(q_lobbies: Query<&Lobby>, mut lobby_func: ResMut<LobbyListFunc>) {
    lobby_func.lobbies.clear();

    for lobby in q_lobbies.iter() {
        let player_count = lobby.len();
        lobby_func.lobbies.push(player_count as u32);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "lobby_list", layer = 1)]
struct LobbyListFunc {
    lobbies: Vec<u32>,
}

#[derive(TypstPath)]
#[typst_path = "typst/server/lobby_list.typ"]
struct LobbyListUi;
