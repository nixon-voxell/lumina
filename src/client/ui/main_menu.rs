use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::client::multiplayer_lobby::MatchmakeState;
use crate::client::Connection;
use crate::protocol::{Matchmake, ReliableChannel};
use crate::ui::main_window::push_to_main_window;
use crate::ui::{interactable_func, pressed, InteractableFunc, InteractionQuery};

use super::lobby::LobbyFunc;
use super::Screen;

pub(super) struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MainMenuUi>()
            .compile_typst_func::<MainMenuUi, MainMenuFunc>()
            .insert_resource(MainMenuFunc {
                connection_msg: "Connecting to server...".to_string(),
                ..default()
            })
            .add_systems(
                Update,
                (
                    push_to_main_window::<MainMenuFunc>(),
                    interactable_func::<MainMenuFunc>,
                    play_btn,
                    reconnect_btn,
                    exit_btn,
                )
                    .run_if(in_state(Screen::MainMenu)),
            )
            .add_systems(OnEnter(Connection::Connected), connected_to_server)
            .add_systems(OnEnter(Connection::Disconnected), disconnected_from_server);
    }
}

fn connected_to_server(mut func: ResMut<MainMenuFunc>) {
    func.connected = true;
}

fn disconnected_from_server(mut func: ResMut<MainMenuFunc>) {
    func.connected = false;
    func.connection_msg = "Disconnected...".to_string();
}

fn play_btn(
    q_interactions: InteractionQuery,
    // mut connection_manager: ResMut<ConnectionManager>,
    // mut lobby_func: ResMut<LobbyFunc>,
    // mut next_lobby_state: ResMut<NextState<MatchmakeState>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    // TODO: Support different player count modes.
    // const PLAYER_COUNT: u8 = 2;

    if pressed(q_interactions.iter(), "btn:play") {
        // lobby_func.curr_player_count = 0;
        // lobby_func.max_player_count = PLAYER_COUNT;

        // let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
        //     &Matchmake(PLAYER_COUNT),
        //     NetworkTarget::None,
        // );

        // next_lobby_state.set(MatchmakeState::Joining);
        next_screen_state.set(Screen::LocalLobby);
    }
}

fn reconnect_btn(
    q_interactions: InteractionQuery,
    mut next_connection_state: ResMut<NextState<Connection>>,
    mut func: ResMut<MainMenuFunc>,
) {
    if pressed(q_interactions.iter(), "btn:reconnect") {
        next_connection_state.set(Connection::Connect);
        func.connection_msg = "Connecting to server...".to_string();
    }
}

fn exit_btn(
    mut commands: Commands,
    q_interactions: InteractionQuery,
    mut app_exit: EventWriter<AppExit>,
) {
    if pressed(q_interactions.iter(), "btn:exit-game") {
        commands.disconnect_client();
        app_exit.send(AppExit::Success);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main_menu", layer = 1)]
pub struct MainMenuFunc {
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
    #[typst_func(named)]
    connected: bool,
    #[typst_func(named)]
    connection_msg: String,
}

impl InteractableFunc for MainMenuFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/main_menu.typ"]
struct MainMenuUi;
