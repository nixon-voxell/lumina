use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use crate::Connection;

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

fn play_btn(interactions: InteractionQuery, mut next_screen_state: ResMut<NextState<Screen>>) {
    if interactions.pressed("btn:play") {
        next_screen_state.set(Screen::LocalLobby);
    }
}

fn reconnect_btn(
    interactions: InteractionQuery,
    mut next_connection_state: ResMut<NextState<Connection>>,
    mut func: ResMut<MainMenuFunc>,
) {
    if interactions.pressed("btn:reconnect") {
        next_connection_state.set(Connection::Connect);
        func.connection_msg = "Connecting to server...".to_string();
    }
}

fn exit_btn(
    mut commands: Commands,
    interactions: InteractionQuery,
    mut app_exit: EventWriter<AppExit>,
) {
    if interactions.pressed("btn:exit-game") {
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
