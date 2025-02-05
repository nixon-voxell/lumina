use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use super::Screen;

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
    commands
        .spawn(LocalLobbyBundle::default())
        .with_children(|builder| {
            builder.spawn((LobbyType::Local.info(), SpawnBlueprint));

            // Spaceship
            builder.spawn((
                SpaceshipType::Defender.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
            ));

            // Weapon
            builder.spawn((
                WeaponType::Cannon.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
            ));

            // Action
            builder.spawn((
                InputManagerBundle::with_map(PlayerAction::input_map()),
                PlayerId::LOCAL,
            ));
        });

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
    world_id: WorldIdx,
}

impl Default for LocalLobbyBundle {
    fn default() -> Self {
        Self {
            local_lobby: LocalLobby,
            spatial: SpatialBundle::default(),
            source: SourceEntity,
            world_id: WorldIdx::default(),
        }
    }
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct LocalLobby;
