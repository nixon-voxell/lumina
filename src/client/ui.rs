use bevy::{color::palettes::css, prelude::*, render::view::RenderLayers, window::PrimaryWindow};
use lightyear::prelude::client::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::protocol::{ExitLobby, JoinLobby, Lobbies, ReliableChannel};

use super::{
    lobby::{LobbyState, MyLobbyId},
    Connection, MyClientId,
};

pub(super) struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MainMenuUi>()
            .compile_typst_func::<MainMenuUi, MainMenuFunc>()
            .render_typst_func::<MainMenuFunc>()
            .init_resource::<MainMenuFunc>()
            .add_systems(Update, (main_menu_window, main_menu_interactions));
        // .add_systems(OnEnter(Connection::Disconnected), connect_server_ui)
        // .add_systems(OnEnter(Connection::Connect), connecting_server_ui)
        // .add_systems(
        //     Update,
        //     (
        //         join_lobby_ui.run_if(in_state(LobbyState::None)),
        //         lobby_ui.run_if(in_state(LobbyState::Joined)),
        //     ),
        // )
        // .add_systems(Update, (connect_server_btn, join_lobby_btn, exit_lobby_btn))
    }
}

fn main_menu_window(
    q_window: Query<Ref<Window>, (With<PrimaryWindow>, Changed<Window>)>,
    mut main_func: ResMut<MainMenuFunc>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    main_func.width = window.width() as f64;
    main_func.height = window.height() as f64;
}

fn main_menu_interactions(
    q_interactions: Query<(&Interaction, &TypstLabel), Changed<Interaction>>,
    mut main_func: ResMut<MainMenuFunc>,
    time: Res<Time>,
) {
    for (interaction, label) in q_interactions.iter() {
        if *interaction == Interaction::Hovered {
            main_func.btn_highlight = label.as_str().to_owned();
            main_func.animate = 0.0;
        } else {
            main_func.btn_highlight.clear();
        }
    }

    const SPEED: f64 = 8.0;
    // Clamp at 1.0
    main_func.animate = f64::min(main_func.animate + time.delta_seconds_f64() * SPEED, 1.0);
}

#[derive(Resource, Default)]
pub struct MainMenuFunc {
    width: f64,
    height: f64,
    btn_highlight: String,
    animate: f64,
}

impl TypstFunc for MainMenuFunc {
    fn func_name(&self) -> &str {
        "main_menu"
    }

    fn render_layers(&self) -> RenderLayers {
        RenderLayers::layer(1)
    }

    fn content(&self, func: foundations::Func) -> Content {
        elem::context(func, |args| {
            args.push(self.width);
            args.push(self.height);
            args.push_named("btn_highlight", self.btn_highlight.clone());
            args.push_named("animate", self.animate);
        })
        .pack()
    }
}

struct MainMenuUi;

impl TypstPath for MainMenuUi {
    fn path() -> &'static str {
        "typst/client/main_menu.typ"
    }
}
