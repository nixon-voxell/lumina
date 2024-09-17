use bevy::{prelude::*, render::view::RenderLayers, window::PrimaryWindow};
use velyst::{prelude::*, typst_element::prelude::*};

use super::lobby::Lobby;

pub(super) struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<ServerUi>()
            .compile_typst_func::<ServerUi, MainFunc>()
            .render_typst_func::<MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(Update, window)
            .add_systems(Update, lobbies);
    }
}

fn window(
    q_window: Query<Ref<Window>, (With<PrimaryWindow>, Changed<Window>)>,
    mut main_func: ResMut<MainFunc>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    main_func.width = window.width() as f64;
    main_func.height = window.height() as f64;
}

fn lobbies(q_lobbies: Query<&Lobby>, mut main_func: ResMut<MainFunc>) {
    main_func.lobbies.clear();

    for lobby in q_lobbies.iter() {
        let player_count = lobby.len();
        main_func.lobbies.push(player_count as u32);
    }
}

#[derive(Resource, Default)]
struct MainFunc {
    width: f64,
    height: f64,
    lobbies: Vec<u32>,
}

impl TypstFunc for MainFunc {
    fn func_name(&self) -> &str {
        "main"
    }

    fn render_layers(&self) -> RenderLayers {
        RenderLayers::layer(1)
    }

    fn content(&self, func: foundations::Func) -> Content {
        elem::context(func, |args| {
            args.push(self.width);
            args.push(self.height);
            args.push(self.lobbies.clone());
        })
        .pack()
    }
}

struct ServerUi;

impl TypstPath for ServerUi {
    fn path() -> &'static str {
        "typst/server/main.typ"
    }
}
