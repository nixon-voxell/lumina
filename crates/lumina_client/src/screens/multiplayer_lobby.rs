use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use super::Screen;

pub(super) struct MultiplayerLobbyPlugin;

impl Plugin for MultiplayerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::MultiplayerLobby), spawn_lobby)
            .add_systems(OnExit(Screen::MultiplayerLobby), despawn_lobby)
            .add_systems(
                Update,
                start_game.run_if(in_state(Screen::MultiplayerLobby)),
            );
    }
}

/// Wait for [`StartGame`] command from server.
fn start_game(
    mut commands: Commands,
    mut start_game_evr: EventReader<MessageEvent<StartGame>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    for _ in start_game_evr.read() {
        // Spawn map and move in to in game screen.
        commands.spawn((MapType::AbandonedFactory.info(), SpawnBlueprint));
        next_screen_state.set(Screen::InGame);
    }
}

/// Spawn lobby scene.
fn spawn_lobby(mut commands: Commands, mut transparency_evr: EventWriter<MainWindowTransparency>) {
    commands.spawn((
        LobbyType::Multiplayer.info(),
        SpawnBlueprint,
        MultiplayerLobby,
    ));
    transparency_evr.send(MainWindowTransparency(1.0));
}

/// Despawn lobby scene.
fn despawn_lobby(mut commands: Commands, q_lobby: Query<Entity, With<MultiplayerLobby>>) {
    let lobby = q_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct MultiplayerLobby;
