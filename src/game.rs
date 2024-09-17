use bevy::prelude::*;

pub mod input;
pub mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Game plugins
        app.init_state::<GameState>()
            .add_plugins((input::InputPlugin, player::PlayerPlugin));
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    None,
    InGame,
}
