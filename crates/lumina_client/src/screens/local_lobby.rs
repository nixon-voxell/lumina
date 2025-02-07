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

use crate::ui::spaceship_select::ClientSpaceshipSelection;
pub(super) struct LocalLobbyPlugin;

// Define the marker component
#[derive(Component)]
struct SpaceshipEntityMarker;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(
            Update,
            (
                update_spaceship_config.run_if(in_state(Screen::LocalLobby)),
                cleanup_marked_entities,
            ),
        )
        .add_systems(OnExit(Screen::LocalLobby), despawn_lobby);
    }
}

/// Spawn lobby scene.
fn spawn_lobby(
    mut commands: Commands,
    selected_ship: Res<ClientSpaceshipSelection>,
    mut transparency_evw: EventWriter<MainWindowTransparency>,
) {
    commands
        .spawn(LocalLobbyBundle::default())
        .with_children(|builder| {
            builder.spawn((LobbyType::Local.info(), SpawnBlueprint));

            // Spaceship
            builder.spawn((
                selected_ship.0.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
                SpaceshipEntityMarker,
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

/// Update spaceship configuration when a new selection is made
fn update_spaceship_config(
    mut commands: Commands,
    selected_ship: Res<ClientSpaceshipSelection>,
    spaceship_query: Query<Entity, With<SpaceshipEntityMarker>>,
    lobby_query: Query<Entity, With<LocalLobby>>,
) {
    if selected_ship.is_changed() {
        // Spawn new spaceship entities for each lobby.
        for lobby in lobby_query.iter() {
            commands.entity(lobby).with_children(|parent| {
                parent.spawn((
                    selected_ship.0.config_info(),
                    SpawnBlueprint,
                    PlayerId::LOCAL,
                    SpaceshipEntityMarker,
                    TransformBundle::default(),
                ));
            });
        }
        // Instead of despawning immediately, mark all existing spaceship entities for removal.
        for spaceship_entity in spaceship_query.iter() {
            commands.entity(spaceship_entity).insert(MarkedForDespawn);
        }
        info!("Updated spaceship to: {:?}", selected_ship.0);
    }
}

fn cleanup_marked_entities(mut commands: Commands, query: Query<Entity, With<MarkedForDespawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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

#[derive(Component)]
struct MarkedForDespawn;
