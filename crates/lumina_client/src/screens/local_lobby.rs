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

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(
            PostUpdate,
            update_spaceship_config.run_if(in_state(Screen::LocalLobby)),
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
                selected_ship.config_info(),
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
    mut select_spaceship_evr: EventReader<SelectSpaceship>,
    q_spaceships: Query<Entity, With<SpaceshipEntityMarker>>,
    q_local_lobby: Query<Entity, With<LocalLobby>>,
) {
    for select_spaceship in select_spaceship_evr.read() {
        // Spawn new spaceship entities for each lobby.
        if let Ok(lobby) = q_local_lobby.get_single() {
            commands.entity(lobby).with_children(|parent| {
                parent.spawn((
                    select_spaceship.0.config_info(),
                    SpawnBlueprint,
                    PlayerId::LOCAL,
                    SpaceshipEntityMarker,
                    TransformBundle::default(),
                ));
            });
        }

        for spaceship_entity in q_spaceships.iter() {
            commands.entity(spaceship_entity).despawn_recursive();
        }

        info!("Updated spaceship to: {:?}", **select_spaceship);
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

/// Tag for the parent entity of the lobby scene.
#[derive(Component, Default)]
pub(super) struct LocalLobby;

// Define the marker component.
#[derive(Component)]
struct SpaceshipEntityMarker;
