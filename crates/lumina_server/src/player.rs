use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use super::lobby::Lobby;
use super::LobbyInfos;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                spawn_players,
                replicate_actions.after(MainSet::EmitEvents),
                replicate_action_spawn.in_set(ServerReplicationSet::ClientReplication),
                replicate_spawn::<Spaceship>,
                replicate_spawn::<Weapon>,
            ),
        );
    }
}

fn spawn_players(
    mut commands: Commands,
    q_new_players: Query<(&PlayerClient, Entity), Added<PlayerClient>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
) {
    for (
        &PlayerClient {
            client_id,
            lobby_entity,
        },
        entity,
    ) in q_new_players.iter()
    {
        // Spawn spaceship.
        commands
            .spawn((
                PlayerId(client_id),
                // TODO: Allow player to choose what spaceship to spawn.
                SpaceshipType::Assassin.config_info(),
                SpawnBlueprint,
            ))
            .set_parent(lobby_entity);

        // Spawn weapon.
        commands
            .spawn((
                PlayerId(client_id),
                // TODO: Allow player to choose what weapon to spawn.
                WeaponType::Cannon.config_info(),
                SpawnBlueprint,
            ))
            .set_parent(lobby_entity);

        let room_id = lobby_entity.room_id();
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &LobbyData { room_id },
            room_id,
            &room_manager,
        );

        info!("SERVER: Spawned player for {client_id}");

        commands.entity(entity).despawn();
    }
}

fn replicate_spawn<T: Component>(
    mut commands: Commands,
    q_entities: Query<(&PlayerId, Entity), (With<T>, Without<SyncTarget>)>,
    lobby_infos: Res<LobbyInfos>,
    mut player_infos: ResMut<PlayerInfos>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (id, entity) in q_entities.iter() {
        let client_id = id.0;
        let Some(room_id) = lobby_infos.get_room_id(&client_id) else {
            error!("Unable to get room id for {client_id}");
            return;
        };

        commands.entity(entity).insert(Replicate {
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

        room_manager.add_entity(entity, room_id);
        player_infos[PlayerInfoType::Spaceship].insert(*id, entity);
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

#[derive(Component)]
pub struct PlayerClient {
    pub client_id: ClientId,
    pub lobby_entity: Entity,
}
