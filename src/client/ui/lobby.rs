use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*, typst_vello::Layer};

use crate::{
    client::lobby::LobbyState,
    protocol::{ExitLobby, ReliableChannel},
    ui::{
        interactable_func, pressed, windowed_func, InteractableFunc, InteractionQuery, WindowedFunc,
    },
};

use super::{main_menu::MainMenuFunc, state_scoped_scene, Screen};

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
            )
            .add_systems(OnEnter(LobbyState::Joining), enter_lobby)
            .add_systems(OnEnter(LobbyState::None), exit_lobby);

        state_scoped_scene::<LobbyFunc>(app, Screen::MultiplayerLobby);
    }
}

// struct TestSceneProcessor;

// impl SceneProcesser for TestSceneProcessor {
//     fn process_scene(&self, _: usize, scene: &SceneKind) -> bevy_vello::vello::Scene {
//         match scene {
//             SceneKind::Shape(shape) => {

//             },
//             SceneKind::Text(_) => todo!(),
//             SceneKind::Image(_) => todo!(),
//         }
//     }
// }

fn setup(mut scene: ResMut<VelystScene<LobbyFunc>>) {
    scene.visibility = Visibility::Hidden;
}

fn exit_lobby_btn(
    q_interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    if pressed(q_interactions.iter(), "btn:exit-lobby") {
        let _ = connection_manager
            .send_message_to_target::<ReliableChannel, _>(&ExitLobby, NetworkTarget::None);

        next_lobby_state.set(LobbyState::None);
        next_screen_state.set(Screen::MainMenu);
    }
}

fn enter_lobby(
    mut lobby_scene: ResMut<VelystScene<LobbyFunc>>,
    mut menu_scene: ResMut<VelystScene<MainMenuFunc>>,
) {
    lobby_scene.visibility = Visibility::Inherited;
    menu_scene.visibility = Visibility::Hidden;
}

fn exit_lobby(
    mut lobby_scene: ResMut<VelystScene<LobbyFunc>>,
    mut menu_scene: ResMut<VelystScene<MainMenuFunc>>,
) {
    lobby_scene.visibility = Visibility::Hidden;
    menu_scene.visibility = Visibility::Inherited;

    lobby_scene.post_process_map.insert(
        TypLabel::new("btn:exit-lobby"),
        velyst::typst_vello::PostProcess {
            layer: Some(Layer {
                alpha: 0.1,
                ..default()
            }),
            ..default()
        },
    );
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "lobby", layer = 1)]
pub struct LobbyFunc {
    width: f64,
    height: f64,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
    #[typst_func(named)]
    pub curr_player_count: u8,
    #[typst_func(named)]
    pub max_player_count: u8,
    #[typst_func(named)]
    pub room_id: Option<u64>,
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

#[derive(TypstPath)]
#[typst_path = "typst/client/lobby.typ"]
struct LobbyUi;
