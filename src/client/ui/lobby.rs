use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::{ExitLobby, ReliableChannel},
    ui::{
        interactable_func, pressed, windowed_func, InteractableFunc, InteractionQuery, WindowedFunc,
    },
};

use super::main_menu::MainMenuFunc;

pub(super) struct LobbyUiPlugin;

impl Plugin for LobbyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyUi>()
            .compile_typst_func::<LobbyUi, LobbyFunc>()
            .render_typst_func::<LobbyFunc>()
            .init_resource::<LobbyFunc>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    windowed_func::<LobbyFunc>,
                    interactable_func::<LobbyFunc>,
                    exit_lobby_btn,
                ),
            );
    }
}

fn setup(mut scene: ResMut<VelystScene<LobbyFunc>>) {
    scene.visibility = Visibility::Hidden;
}

fn exit_lobby_btn(
    q_interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut lobby_scene: ResMut<VelystScene<LobbyFunc>>,
    mut menu_scene: ResMut<VelystScene<MainMenuFunc>>,
) {
    if pressed(q_interactions.iter(), "btn:exit-lobby") {
        lobby_scene.visibility = Visibility::Hidden;
        menu_scene.visibility = Visibility::Inherited;

        let _ = connection_manager
            .send_message_to_target::<ReliableChannel, _>(&ExitLobby, NetworkTarget::None);
    }
}

#[derive(Resource, Default)]
pub struct LobbyFunc {
    width: f64,
    height: f64,
    hovered_button: Option<TypLabel>,
    hovered_animation: f64,
    pub curr_player_count: u8,
    pub max_player_count: u8,
    pub room_id: Option<u64>,
}

impl TypstFunc for LobbyFunc {
    fn func_name(&self) -> &str {
        "lobby"
    }

    fn content(&self, func: foundations::Func) -> Content {
        elem::context(func, |args| {
            args.push(self.width);
            args.push(self.height);
            args.push_named("hovered_button", self.hovered_button);
            args.push_named("hovered_animation", self.hovered_animation);
            args.push_named("curr_player_count", self.curr_player_count);
            args.push_named("max_player_count", self.max_player_count);
            args.push_named("room_id", self.room_id);
        })
        .pack()
    }
}

impl WindowedFunc for LobbyFunc {
    fn set_width_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

impl InteractableFunc for LobbyFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

struct LobbyUi;

impl TypstPath for LobbyUi {
    fn path() -> &'static str {
        "typst/client/lobby.typ"
    }
}
