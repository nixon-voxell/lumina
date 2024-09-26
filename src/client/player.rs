use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{PlayerAction, ReplicateInputBundle};
use crate::shared::player::{shared_handle_player_movement, PlayerId, PlayerMovement};
use crate::shared::FixedSet;

use super::lobby::LobbyState;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_player_spawn, convert_3d_mesh_to_2d))
            .add_systems(Update, convert_3d_mesh_to_2d)
            .add_systems(FixedUpdate, handle_player_movement.in_set(FixedSet::Main))
            .add_systems(OnEnter(LobbyState::None), despawn_input);
    }
}

fn convert_3d_mesh_to_2d(
    mut commands: Commands,
    q_meshes: Query<(&Handle<Mesh>, &Handle<StandardMaterial>, &Name, Entity)>,
    std_materials: Res<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mesh_handle, std_material_handle, name, entity) in q_meshes.iter() {
        let Some(std_material) = std_materials.get(std_material_handle) else {
            continue;
        };

        let color_material = color_materials.add(ColorMaterial {
            color: std_material.base_color,
            texture: std_material.base_color_texture.clone(),
        });

        commands
            .entity(entity)
            .remove::<Handle<Mesh>>()
            .remove::<Handle<StandardMaterial>>()
            .insert((Mesh2dHandle(mesh_handle.clone()), color_material));

        info!("Converted {name:?} into 2d mesh.");
    }
}

/// Add visuals and input for player on player spawn.
fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<
        (&PlayerId, Entity, Has<Predicted>),
        (Or<(Added<Predicted>, Added<Interpolated>)>, With<Position>),
    >,
) {
    for (id, entity, is_predicted) in q_predicted.iter() {
        info!("Spawn predicted entity.");

        // Add visuals for player.
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Player.glb"), // mandatory !!
            SpawnBlueprint,
        ));

        if is_predicted {
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*id));
        }
    }
}

/// Handle player movement on [`PlayerAction`].
fn handle_player_movement(
    q_player: Query<Entity, (With<Predicted>, With<Position>)>,
    q_action_states: Query<
        &ActionState<PlayerAction>,
        (With<PrePredicted>, Changed<ActionState<PlayerAction>>),
    >,
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
