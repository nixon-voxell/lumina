use bevy::{prelude::*, window::PrimaryWindow};
use velyst::{prelude::*, typst_element::prelude::*};

use crate::{
    protocol::ReliableChannel,
    ui::{pressed, InteractionQuery, WindowQuery},
};

pub(super) struct MatchmakingUiPlugin;

impl Plugin for MatchmakingUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<MatchmakingUi>()
            .compile_typst_func::<MatchmakingUi, MatchmakingFunc>()
            .render_typst_func::<MatchmakingFunc>()
            .init_resource::<MatchmakingFunc>()
            .add_systems(Update, window);
    }
}

fn window(q_window: WindowQuery, mut func: ResMut<MatchmakingFunc>) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.width = window.width() as f64;
    func.height = window.height() as f64;
}

#[derive(Resource, Default)]
struct MatchmakingFunc {
    width: f64,
    height: f64,
    player_count: u32,
}

impl TypstFunc for MatchmakingFunc {
    fn func_name(&self) -> &str {
        "matchmaking"
    }

    fn content(&self, func: foundations::Func) -> Content {
        elem::context(func, |args| {
            args.push(self.width);
            args.push(self.height);
            args.push_named("player_count", self.player_count);
        })
        .pack()
    }
}

struct MatchmakingUi;

impl TypstPath for MatchmakingUi {
    fn path() -> &'static str {
        "typst/client/matchmaking.typ"
    }
}
