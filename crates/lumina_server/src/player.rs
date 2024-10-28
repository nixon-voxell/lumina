use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::spaceship::{Spaceship, SpaceshipType};
use lumina_shared::prelude::*;
use lumina_shared::procedural_map::grid_map::{find_valid_spawn_points, ValidSpawnPoints};
use server::*;

use super::lobby::Lobby;
use super::LobbyInfos;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                //detect_new_spaceships.after(find_valid_spawn_points),
                replicate_actions.after(MainSet::EmitEvents),
                replicate_action_spawn.in_set(ServerReplicationSet::ClientReplication),
                replicate_spaceship_spawn,
            ),
        );
        app.add_systems(
            Update,
            (detect_new_spaceships.after(find_valid_spawn_points),),
        );
    }
}

/// Spawn an entity for a given client.
pub(super) fn spawn_player_entity(commands: &mut Commands, client_id: ClientId) {
    info!("SERVER: Spawn player for {:?}", client_id);

    commands.spawn((
        PlayerId(client_id),
        // TODO: Allow player to choose what spaceship to spawn.
        SpaceshipType::Assassin.config_info(),
        SpawnBlueprint,
    ));
}

/// System to detect new spaceship spawns and assign them to valid positions.
fn detect_new_spaceships(
    mut query: Query<Entity, (With<Spaceship>, Added<SourceEntity>)>,
    valid_spawn_points: Res<ValidSpawnPoints>,
    mut commands: Commands,
) {
    // Ensure we have available spawn points.
    if valid_spawn_points.0.is_empty() {
        warn!("No valid spawn points available!");
        return;
    }

    let mut spawn_points_iter = valid_spawn_points.0.iter().cycle(); // Cycle through spawn points.

    for entity in query.iter_mut() {
        if let Some(&(x, y)) = spawn_points_iter.next() {
            // Convert (x, y) to a Vec3 position.
            let position = Vec3::new(x as f32, y as f32, 0.0);
            let rotation = Quat::IDENTITY; // Default rotation.

            // Insert transform components into the entity.
            commands.entity(entity).insert((
                Transform {
                    translation: position,
                    rotation,
                    ..Default::default()
                },
                GlobalTransform::default(),
            ));

            info!("Spawned spaceship at position: ({}, {})", x, y);
        } else {
            warn!("Ran out of spawn points!");
        }
    }
}

fn replicate_spaceship_spawn(
    mut commands: Commands,
    q_spaceships: Query<(&PlayerId, Entity), (With<Spaceship>, Without<SyncTarget>)>,
    lobby_infos: Res<LobbyInfos>,
    mut player_infos: ResMut<PlayerInfos>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (id, spaceship_entity) in q_spaceships.iter() {
        let client_id = id.0;
        let Some(room_id) = lobby_infos.get_room_id(&client_id) else {
            error!("Unable to get room id for {client_id}");
            return;
        };

        commands.entity(spaceship_entity).insert(Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            relevance_mode: NetworkRelevanceMode::InterestManagement,
            ..default()
        });

        room_manager.add_entity(spaceship_entity, room_id);
        player_infos[PlayerInfoType::Spaceship].insert(*id, spaceship_entity);
    }
}

/// Replicate action back to other clients.
fn replicate_action_spawn(
    mut commands: Commands,
    q_actions: Query<(&PlayerId, Entity), (Added<ActionState<PlayerAction>>, Added<Replicated>)>,
    lobby_infos: Res<LobbyInfos>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (id, entity) in q_actions.iter() {
        let client_id = id.0;

        if let Some(room_id) = lobby_infos.get_room_id(&client_id) {
            info!("SERVER: Received input spawn from {client_id} in room: {room_id:?}");
            let replicate = Replicate {
                sync: SyncTarget {
                    // Allow a client to predict other client's input.
                    prediction: NetworkTarget::All,
                    ..default()
                },
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                group: INPUT_REPLICATION_GROUP,
                relevance_mode: NetworkRelevanceMode::InterestManagement,
                ..default()
            };

            commands.entity(entity).insert((
                replicate,
                // If we receive a pre-predicted entity, only send the
                // prepredicted component back to the original client.
                OverrideTargetComponent::<PrePredicted>::new(NetworkTarget::Single(client_id)),
            ));
            room_manager.add_entity(entity, room_id);
        }
    }
}

/// Replicate the inputs (actions) of a client to other clients
/// so that a client can predict other clients.
fn replicate_actions(
    q_lobbies: Query<&Lobby>,
    mut connection: ResMut<ConnectionManager>,
    mut action_evr: EventReader<MessageEvent<InputMessage<PlayerAction>>>,
    lobby_infos: Res<LobbyInfos>,
) {
    for event in action_evr.read() {
        let inputs = event.message();
        let client_id = event.context();

        let Some(&lobby_entity) = lobby_infos.get(client_id) else {
            continue;
        };

        let Ok(lobby) = q_lobbies.get(lobby_entity) else {
            continue;
        };

        // OPTIONAL: Do some validation on the inputs to check that there's no cheating

        // Rebroadcast the input to other clients inside the lobby.
        for client_id in lobby.iter().filter(|id| *id != client_id) {
            let _ = connection.send_message::<InputChannel, _>(*client_id, inputs);
        }
    }
}
