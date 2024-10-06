use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{PlayerAction, ReplicateInputBundle};
use crate::shared::player::{shared_handle_player_movement, PlayerId, PlayerMovement, SpaceShip};
use crate::shared::MovementSet;

use super::lobby::LobbyState;
use super::MyClientId;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_player_spawn.run_if(resource_exists::<MyClientId>),
                convert_3d_to_2d_mesh,
                convert_std_to_color_material,
            ),
        )
        .add_systems(Update, convert_3d_to_2d_mesh)
        .add_systems(
            FixedUpdate,
            handle_player_movement.in_set(MovementSet::Input),
        )
        .add_systems(OnEnter(LobbyState::None), despawn_input);
    }
}

fn convert_3d_to_2d_mesh(mut commands: Commands, q_meshes: Query<(&Handle<Mesh>, &Name, Entity)>) {
    for (mesh_handle, name, entity) in q_meshes.iter() {
        commands
            .entity(entity)
            .remove::<Handle<Mesh>>()
            .insert(Mesh2dHandle(mesh_handle.clone()));

        info!("Converted {name:?} 3d mesh into 2d mesh.");
    }
}

fn convert_std_to_color_material(
    mut commands: Commands,
    q_meshes: Query<(&Handle<StandardMaterial>, &Name, Entity)>,
    std_materials: Res<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (std_material, name, entity) in q_meshes.iter() {
        let Some(std_material) = std_materials.get(std_material) else {
            continue;
        };

        let color_material = color_materials.add(ColorMaterial {
            color: std_material.base_color,
            texture: std_material.base_color_texture.clone(),
        });

        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>()
            .insert(color_material);

        info!("Converted {name:?} standard material into color material.");
    }
}

/// Add visuals and input for player on player spawn.
fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<
        (&PlayerId, Entity, Has<Predicted>),
        (
            // Or<(Added<Predicted>, Added<Interpolated>)>,
            Added<Predicted>,
            Added<SpaceShip>,
        ),
    >,
    my_client_id: Res<MyClientId>,
) {
    for (player_id, entity, _is_predicted) in q_predicted.iter() {
        info!("Spawn predicted entity ({:?}).", player_id);

        // Add visuals for player.
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Player.glb"),
            SpawnBlueprint,
        ));

        if player_id.0 == my_client_id.0 {
            commands.entity(entity).insert(MyPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*player_id));
        }
    }
}

// TODO: Simulate other client's actions as well.
/// Handle player movement on [`PlayerAction`].
fn handle_player_movement(
    q_player: Query<Entity, With<MyPlayer>>,
    q_action_states: Query<&ActionState<PlayerAction>, With<PrePredicted>>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
) {
    let Ok(action_state) = q_action_states.get_single() else {
        return;
    };

    let Ok(player_entity) = q_player.get_single() else {
        return;
    };

    shared_handle_player_movement(action_state, player_entity, &mut player_movement_evw);
}

/// Despawn all player inputs.
fn despawn_input(
    mut commands: Commands,
    q_action_states: Query<Entity, With<ActionState<PlayerAction>>>,
) {
    for entity in q_action_states.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub(super) struct MyPlayer;
