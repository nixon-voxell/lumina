use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::Connection;

use super::Screen;

pub(super) struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MainMenuUi>()
            .compile_typst_func::<MainMenuUi, MainMenuFunc>()
            .push_to_main_window::<MainMenuUi, MainMenuFunc, _>(
                MainWindowSet::Default,
                in_state(Screen::MainMenu),
            )
            .recompile_on_interaction::<MainMenuFunc>(|func| &mut func.dummy_update)
            .insert_resource(MainMenuFunc {
                connection_msg: "Connecting to server...".to_string(),
                ..default()
            })
            .add_systems(OnEnter(Screen::MainMenu), main_window_transparency)
            .add_systems(
                Update,
                (play_btn, reconnect_btn, exit_btn).run_if(in_state(Screen::MainMenu)),
            )
            .add_systems(OnEnter(Connection::Connected), connected_to_server)
            .add_systems(OnEnter(Connection::Disconnected), disconnected_from_server);
    }
}

fn connected_to_server(mut func: ResMut<MainMenuFunc>) {
    func.connected = true;
}

pub fn disconnected_from_server(
    mut func: ResMut<MainMenuFunc>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    func.connected = false;
    func.connection_msg = "Disconnected...".to_string();
    next_screen_state.set(Screen::MainMenu);
}

fn main_window_transparency(mut evw_transparency: EventWriter<MainWindowTransparency>) {
    evw_transparency.send(MainWindowTransparency(0.0));
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
        next_connection_state.set(Connection::Connecting);
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
    connected: bool,
    connection_msg: String,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/main_menu.typ"]
struct MainMenuUi;
