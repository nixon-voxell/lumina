use bevy::prelude::*;
use lumina_terrain::prelude::*;
use lumina_ui::prelude::*;

use super::ui::Screen;

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby);
        app.add_systems(OnExit(Screen::MultiplayerLobby), despawn_terrain);
    }
}

fn spawn_lobby(
    mut _commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    // commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
    **main_window_transparency = 1.0;
}

fn despawn_terrain(mut commands: Commands) {
    // grid_map.deinitialize_tiles(&mut commands);
}
