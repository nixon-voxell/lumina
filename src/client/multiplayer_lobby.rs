use bevy::prelude::*;
// use blenvy::*;

use super::ui::Screen;

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby);
    }
}

fn spawn_lobby(mut _commands: Commands) {
    // commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
}
