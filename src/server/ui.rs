use bevy::prelude::*;
use velyst::prelude::*;

use crate::ui::{windowed_func, WindowedFunc};

use super::lobby::Lobby;

pub(super) struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<ServerUi>()
            .compile_typst_func::<ServerUi, MainFunc>()
            .render_typst_func::<MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(Update, windowed_func::<MainFunc>)
            .add_systems(Update, lobbies);
    }
}

fn lobbies(q_lobbies: Query<&Lobby>, mut main_func: ResMut<MainFunc>) {
    main_func.lobbies.clear();

    for lobby in q_lobbies.iter() {
        let player_count = lobby.len();
        main_func.lobbies.push(player_count as u32);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    width: f64,
    height: f64,
    lobbies: Vec<u32>,
}

impl WindowedFunc for MainFunc {
    fn set_width_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/server/main.typ"]
struct ServerUi;
