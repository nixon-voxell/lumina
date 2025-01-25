use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use super::lobby::Lobby;
use super::LobbyInfos;

pub(super) mod objective;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(objective::ObjectivePlugin)
            .add_event::<SpawnClientPlayer>()
            .add_systems(
                PreUpdate,
                (
                    replicate_actions.before(MainSet::Send),
                    replicate_action_spawn.in_set(ServerReplicationSet::ClientReplication),
                    replicate_item_spawn::<Spaceship>,
                    replicate_item_spawn::<Weapon>,
                ),
            )
            .observe(spawn_players);
    }
}

fn spawn_players(trigger: Trigger<SpawnClientPlayer>, mut commands: Commands) {
    let &SpawnClientPlayer {
        client_id,
        world_entity,
    } = trigger.event();

    // Spawn spaceship.
    commands
        .spawn((
            PlayerId(client_id),
            // TODO: Allow player to choose what spaceship to spawn.
            SpaceshipType::Assassin.config_info(),
            SpawnBlueprint,
        ))
        .set_parent(world_entity);

    // Spawn weapon.
    commands
        .spawn((
            PlayerId(client_id),
            // TODO: Allow player to choose what weapon to spawn.
            WeaponType::Cannon.config_info(),
            SpawnBlueprint,
        ))
        .set_parent(world_entity);

    info!("SERVER: Spawned player for {client_id}");
}

/// Replicate a player's item back other clients while granting
/// prediction to all clients and no interpolation to target client.
fn replicate_item_spawn<T: Component>(
    mut commands: Commands,
    q_entities: Query<(&PlayerId, Entity), (With<T>, Without<SyncTarget>)>,
    lobby_infos: Res<LobbyInfos>,
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
    }
}

/// Replicate action back to other clients with prediction turned on for all clients.
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

#[derive(Event)]
pub struct SpawnClientPlayer {
    pub client_id: ClientId,
    /// The entity that holds the world of the client.
    pub world_entity: Entity,
}
