use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use super::ui::Screen;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(OnExit(Screen::LocalLobby), despawn_lobby);
    }
}

/// Spawn lobby scene.
fn spawn_lobby(mut commands: Commands, mut transparency_evw: EventWriter<MainWindowTransparency>) {
    let lobby_scene = commands.spawn(LocalLobbyBundle::default()).id();
    commands
        .spawn((LobbyType::Local.info(), SpawnBlueprint))
        .set_parent(lobby_scene);

    // Spaceship
    commands
        .spawn((
            SpaceshipType::Assassin.config_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    // Weapon
    commands
        .spawn((
            WeaponType::Cannon.config_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    // Action
    commands
        .spawn((
            InputManagerBundle::with_map(PlayerAction::input_map()),
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    transparency_evw.send(MainWindowTransparency(1.0));
}

/// Despawn lobby scene.
fn despawn_lobby(mut commands: Commands, q_lobby: Query<Entity, With<LocalLobby>>) {
    let lobby = q_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, With<Predicted>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Bundle)]
pub(super) struct LocalLobbyBundle {
    local_lobby: LocalLobby,
    spatial: SpatialBundle,
    source: SourceEntity,
    world_id: PhysicsWorldId,
}

impl Default for LocalLobbyBundle {
    fn default() -> Self {
        Self {
            local_lobby: LocalLobby,
            spatial: SpatialBundle::default(),
            source: SourceEntity,
            world_id: PhysicsWorldId::default(),
        }
    }
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct LocalLobby;
