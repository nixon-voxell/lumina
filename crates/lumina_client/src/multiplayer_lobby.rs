use bevy::prelude::*;
use lumina_ui::prelude::*;

use super::ui::Screen;
use lumina_shared::procedural_map::grid_map::{GridMap, Tile};

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby);
        app.add_systems(OnExit(Screen::MultiplayerLobby), despawn_grid);
    }
}

fn spawn_lobby(
    mut _commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    // commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
    **main_window_transparency = 1.0;
}

fn despawn_grid(
    mut commands: Commands,
    grid_map: Res<GridMap>, // Add this line
) {
    grid_map.deinitialize_tiles(&mut commands);
}
