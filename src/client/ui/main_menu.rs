use bevy::{prelude::*, render::view::RenderLayers};
use client::*;
use lightyear::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::client::Connection;
use crate::protocol::{Matchmake, ReliableChannel};
use crate::ui::{pressed, InteractionQuery, WindowQuery};

use super::lobby::LobbyFunc;

pub(super) struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MainMenuUi>()
            .compile_typst_func::<MainMenuUi, MainMenuFunc>()
            .render_typst_func::<MainMenuFunc>()
            .init_resource::<MainMenuFunc>()
            .add_systems(Update, (window, main_menu_hover))
            .add_systems(Update, (play_btn, reconnect_btn, exit_btn))
            .add_systems(OnEnter(Connection::Connected), connected_to_server)
            .add_systems(OnEnter(Connection::Disconnected), disconnected_from_server);
    }
}

fn window(q_window: WindowQuery, mut func: ResMut<MainMenuFunc>) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.width = window.width() as f64;
    func.height = window.height() as f64;
}

fn main_menu_hover(
    q_interactions: Query<(&Interaction, &TypstLabel)>,
    mut func: ResMut<MainMenuFunc>,
    time: Res<Time>,
    mut last_hovered: Local<String>,
) {
    func.hovered_button.clear();
    for (interaction, label) in q_interactions.iter() {
        if *interaction == Interaction::Hovered {
            func.hovered_button = label.as_str().to_owned();
        }
    }

    if func.hovered_button != *last_hovered {
        func.animate = 0.0;
        *last_hovered = func.hovered_button.clone();
    }

    const SPEED: f64 = 8.0;
    // Clamp at 1.0
    func.animate = f64::min(func.animate + time.delta_seconds_f64() * SPEED, 1.0);
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
    mut menu_scene: ResMut<VelystScene<MainMenuFunc>>,
    mut lobby_scene: ResMut<VelystScene<LobbyFunc>>,
    mut lobby_func: ResMut<LobbyFunc>,
) {
    // TODO: Support different player count modes.
    const PLAYER_COUNT: u8 = 2;

    if pressed(q_interactions.iter(), "btn:play") {
        menu_scene.visibility = Visibility::Hidden;
        lobby_scene.visibility = Visibility::Inherited;
        lobby_func.curr_player_count = 0;
        lobby_func.max_player_count = PLAYER_COUNT;

        let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
            &Matchmake(PLAYER_COUNT),
            NetworkTarget::None,
        );
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

#[derive(Resource, Default)]
pub struct MainMenuFunc {
    width: f64,
    height: f64,
    hovered_button: String,
    animate: f64,
    connected: bool,
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
            args.push_named("hovered_button", self.hovered_button.clone());
            args.push_named("animate", self.animate);
            args.push_named("connected", self.connected);
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
