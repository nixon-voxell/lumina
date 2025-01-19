use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use lumina_ui::prelude::*;

use super::in_game::TerrainEntity;
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
    mut start_game_evr: EventReader<MessageEvent<StartGame>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    terrain_entity: Res<TerrainEntity>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
) {
    for start_game in start_game_evr.read() {
        generate_terrain_evw.send(GenerateTerrain {
            seed: start_game.message().seed,
            entity: **terrain_entity,
            layers: CollisionLayers::ALL,
            world_id: WorldIdx::default(),
        });

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
