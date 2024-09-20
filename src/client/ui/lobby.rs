use bevy::prelude::*;
use velyst::{prelude::*, typst_element::prelude::*};

use crate::ui::{windowed_func, WindowedFunc};

pub(super) struct LobbyUiPlugin;

impl Plugin for LobbyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<LobbyUi>()
            .compile_typst_func::<LobbyUi, LobbyFunc>()
            .render_typst_func::<LobbyFunc>()
            .init_resource::<LobbyFunc>()
            .add_systems(Startup, setup)
            .add_systems(Update, windowed_func::<LobbyFunc>);
    }
}

fn setup(mut scene: ResMut<VelystScene<LobbyFunc>>) {
    scene.visibility = Visibility::Hidden;
}

#[derive(Resource, Default)]
pub struct LobbyFunc {
    width: f64,
    height: f64,
    pub curr_player_count: u8,
    pub max_player_count: u8,
    pub room_id: Option<u64>,
    hovered_button: String,
    animate: f64,
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
            args.push_named("hovered_button", self.hovered_button.clone());
            args.push_named("animate", self.animate);
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

struct LobbyUi;

impl TypstPath for LobbyUi {
    fn path() -> &'static str {
        "typst/client/lobby.typ"
    }
}
