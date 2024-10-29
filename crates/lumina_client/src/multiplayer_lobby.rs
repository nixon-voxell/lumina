use bevy::prelude::*;
use lumina_ui::prelude::*;

use super::ui::Screen;

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby);
    }
}

fn spawn_lobby(
    mut _commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    // commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
    **main_window_transparency = 1.0;
}
