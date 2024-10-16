use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{MovementSet, PlayerAction, ReplicateInputBundle};
use crate::shared::player::{shared_handle_player_movement, PlayerId, PlayerMovement, SpaceShip};
use crate::shared::LocalEntity;

use super::multiplayer_lobby::MatchmakeState;
use super::MyClientId;

mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(weapon::WeaponPlugin)
            .init_resource::<ActionState<PlayerAction>>()
            .insert_resource(PlayerAction::input_map())
            .init_resource::<PlayerMap>()
            .add_systems(
                Update,
                (
                    handle_player_spawn_visual,
                    handle_player_spawn.run_if(resource_exists::<MyClientId>),
                ),
            )
            .add_systems(
                FixedUpdate,
                (handle_player_movement, handle_local_player_movement).in_set(MovementSet::Input),
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
    my_client_id: Res<MyClientId>,
    mut player_map: ResMut<PlayerMap>,
) {
    for (player_id, entity) in q_predicted.iter() {
        info!("Spawned player {:?}.", entity);
        let client_id = player_id.0;

        if client_id == my_client_id.0 {
            // Mark our player.
            commands.entity(entity).insert(MyPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*player_id));
        }

        player_map.insert(client_id, entity);
    }
}

/// Handle player movement based on [`PlayerAction`].
fn handle_player_movement(
    // Handles all predicted player movements too (other clients).
    q_actions: Query<(&PlayerId, &ActionState<PlayerAction>), With<Predicted>>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
    player_map: Res<PlayerMap>,
) {
    for (id, action) in q_actions.iter() {
        if let Some(player_entity) = player_map.get(&id.0) {
            shared_handle_player_movement(action, *player_entity, &mut player_movement_evw);
        }
    }
}

fn handle_local_player_movement(
    q_players: Query<Entity, (With<LocalEntity>, With<SpaceShip>)>,
    action: Res<ActionState<PlayerAction>>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
) {
    for entity in q_players.iter() {
        shared_handle_player_movement(&action, entity, &mut player_movement_evw);
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

/// The player the the local client is controlling.
#[derive(Component)]
pub(super) struct MyPlayer;

/// Maps client id to player entity.
#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct PlayerMap(HashMap<ClientId, Entity>);
