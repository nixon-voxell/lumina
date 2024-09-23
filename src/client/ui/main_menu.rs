use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::client::lobby::LobbyState;
use crate::client::Connection;
use crate::protocol::{Matchmake, ReliableChannel};
use crate::ui::{
    interactable_func, pressed, windowed_func, InteractableFunc, InteractionQuery, WindowedFunc,
};

use super::lobby::LobbyFunc;

pub(super) struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MainMenuUi>()
            .compile_typst_func::<MainMenuUi, MainMenuFunc>()
            .render_typst_func::<MainMenuFunc>()
            .init_resource::<MainMenuFunc>()
            .add_systems(
                Update,
                (
                    windowed_func::<MainMenuFunc>,
                    interactable_func::<MainMenuFunc>,
                ),
            )
            .add_systems(Update, (play_btn, reconnect_btn, exit_btn))
            .add_systems(OnEnter(Connection::Connected), connected_to_server)
            .add_systems(OnEnter(Connection::Disconnected), disconnected_from_server);
    }
}

fn connected_to_server(mut func: ResMut<MainMenuFunc>) {
    func.connected = true;
}

fn disconnected_from_server(mut func: ResMut<MainMenuFunc>) {
    func.connected = false;
}

fn play_btn(
    q_interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut lobby_func: ResMut<LobbyFunc>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    // TODO: Support different player count modes.
    const PLAYER_COUNT: u8 = 2;

    if pressed(q_interactions.iter(), "btn:play") {
        lobby_func.curr_player_count = 0;
        lobby_func.max_player_count = PLAYER_COUNT;

        let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
            &Matchmake(PLAYER_COUNT),
            NetworkTarget::None,
        );

        next_lobby_state.set(LobbyState::Joining);
    }
}

fn reconnect_btn(
    q_interactions: InteractionQuery,
    mut next_connection_state: ResMut<NextState<Connection>>,
) {
    if pressed(q_interactions.iter(), "btn:reconnect") {
        next_connection_state.set(Connection::Connect);
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
    width: f64,
    height: f64,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
    #[typst_func(named)]
    connected: bool,
}

impl WindowedFunc for MainMenuFunc {
    fn set_width_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
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
