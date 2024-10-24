use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use super::Screen;

pub(super) struct LobbyUiPlugin;

impl Plugin for LobbyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyUi>()
            .compile_typst_func::<LobbyUi, LobbyFunc>()
            .init_resource::<LobbyFunc>()
            .add_systems(
                Update,
                (
                    push_to_main_window::<LobbyFunc>(),
                    interactable_func::<LobbyFunc>,
                    exit_lobby_btn,
                )
                    .run_if(in_state(Screen::MultiplayerLobby)),
            );
    }
}

fn exit_lobby_btn(
    interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    if interactions.pressed("btn:exit-lobby") {
        let _ = connection_manager.send_message::<ReliableChannel, _>(&ExitLobby);

        next_screen_state.set(Screen::MainMenu);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "lobby", layer = 1)]
pub struct LobbyFunc {
    hovered_button: Option<TypLabel>,
    hovered_animation: f64,
    pub curr_player_count: u8,
    pub max_player_count: u8,
    pub room_id: Option<u64>,
}

impl InteractableFunc for LobbyFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/lobby.typ"]
struct LobbyUi;
