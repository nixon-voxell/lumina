use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{PlayerAction, ReplicateInputBundle};
use crate::shared::player::{LocalPlayer, PlayerId, PlayerInfo, PlayerInfos, SpaceShip};
use crate::shared::LocalEntity;

use super::multiplayer_lobby::MatchmakeState;
use super::LocalClientId;

mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(weapon::WeaponPlugin)
            .insert_resource(PlayerAction::input_map())
            .add_systems(
                Update,
                (
                    handle_player_spawn_visual,
                    handle_player_spawn.run_if(resource_exists::<LocalClientId>),
                ),
            )
            .add_systems(OnEnter(MatchmakeState::None), despawn_networked_inputs);
    }
}

/// Add visuals for player.
fn handle_player_spawn_visual(
    mut commands: Commands,
    // Handle both networked and local players.
    q_players: Query<Entity, (Added<SpaceShip>, Or<(Added<Predicted>, Added<LocalEntity>)>)>,
) {
    for entity in q_players.iter() {
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Player.glb"),
            SpawnBlueprint,
        ));
    }
}

/// Add visuals and input for player on player spawn.
fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<
        (&PlayerId, Entity),
        (Added<SpaceShip>, Or<(Added<Predicted>, Added<LocalEntity>)>),
    >,
    local_client_id: Res<LocalClientId>,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (player_id, entity) in q_predicted.iter() {
        info!("Spawned player {:?}.", entity);
        let client_id = player_id.0;

        if client_id == local_client_id.0 {
            // Mark our player.
            commands.entity(entity).insert(LocalPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*player_id));
        }

        player_infos.insert(
            client_id,
            PlayerInfo {
                // TODO: Add lobby entity with the correct bits from room id.
                lobby: Entity::PLACEHOLDER,
                player: entity,
                input: None,
            },
        );
    }
}

/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, Without<LocalEntity>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}
