use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;

use crate::shared::{input::LocalInputBundle, player::LocalPlayerBundle};

use super::{player::MyPlayer, ui::Screen};

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::LocalLobby), init_lobby);
    }
}

/// Spawn lobby scene and player.
fn init_lobby(mut commands: Commands) {
    commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
    commands
        .spawn(LocalPlayerBundle::new(
            Position::default(),
            Rotation::radians(std::f32::consts::FRAC_PI_2),
        ))
        .insert(LocalInputBundle::default())
        .insert(MyPlayer);
}
