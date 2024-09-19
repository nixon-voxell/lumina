use bevy::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::ui::WindowQuery;

pub(super) struct LobbyUiPlugin;

impl Plugin for LobbyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyUi>()
            .compile_typst_func::<LobbyUi, LobbyFunc>()
            .render_typst_func::<LobbyFunc>()
            .init_resource::<LobbyFunc>()
            .add_systems(Startup, setup)
            .add_systems(Update, window);
    }
}

fn setup(mut scene: ResMut<VelystScene<LobbyFunc>>) {
    scene.visibility = Visibility::Hidden;
}

fn window(q_window: WindowQuery, mut func: ResMut<LobbyFunc>) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.width = window.width() as f64;
    func.height = window.height() as f64;
}

#[derive(Resource, Default)]
pub struct LobbyFunc {
    width: f64,
    height: f64,
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
            args.push_named("curr_player_count", self.curr_player_count);
            args.push_named("max_player_count", self.max_player_count);
            args.push_named("room_id", self.room_id);
        })
        .pack()
    }
}

struct LobbyUi;

impl TypstPath for LobbyUi {
    fn path() -> &'static str {
        "typst/client/lobby.typ"
    }
}
