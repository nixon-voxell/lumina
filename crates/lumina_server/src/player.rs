use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_shared::protocol::SelectSpaceship;

use server::*;

use super::lobby::Lobby;
use super::LobbyInfos;

pub(super) mod objective;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(objective::ObjectivePlugin)
            .add_event::<SpawnClientPlayer>()
            .init_resource::<ClientSpaceshipSelection>()
            .add_systems(
                PreUpdate,
                (
                    replicate_actions.before(MainSet::Send),
                    replicate_action_spawn.in_set(ServerReplicationSet::ClientReplication),
                    replicate_item_spawn::<Spaceship>,
                    replicate_item_spawn::<Weapon>,
                ),
            )
            .add_systems(
                Update,
                (handle_spaceship_selection, remove_spaceship_selection),
            )
            .observe(spawn_players);
    }
}

/// Cache client's spaceship selection on message received.
fn handle_spaceship_selection(
    mut events: EventReader<MessageEvent<SelectSpaceship>>,
    mut selection: ResMut<ClientSpaceshipSelection>,
) {
    for event in events.read() {
        let client_id = event.context();
        let spaceship = event.message().0;
        selection.insert(*client_id, spaceship);
        info!(
            "Server: Cached spaceship selection for client {}: {:?}",
            client_id, spaceship
        );
    }
}

/// Remove cached spaceship selection on client disconnection.
fn remove_spaceship_selection(
    mut evr_disconnect: EventReader<DisconnectEvent>,
    mut selection: ResMut<ClientSpaceshipSelection>,
) {
    for disconnect in evr_disconnect.read() {
        selection.remove(&disconnect.client_id);
    }
}

fn spawn_players(
    trigger: Trigger<SpawnClientPlayer>,
    mut commands: Commands,
    selections: Res<ClientSpaceshipSelection>,
) {
    let &SpawnClientPlayer {
        client_id,
        world_entity,
    } = trigger.event();

    // Look up the player's selected spaceship; default to Assassin if none was provided.
    let spaceship_type = selections.get(&client_id).copied().unwrap_or_default();

    // Determine the weapon type based on the selected spaceship.
    let weapon_type = match spaceship_type {
        SpaceshipType::Assassin => WeaponType::Cannon,
        SpaceshipType::Defender => WeaponType::GattlingGun,
    };

    // Spawn the spaceship using its configuration.
    commands
        .spawn((
            PlayerId(client_id),
            spaceship_type.config_info(),
            SpawnBlueprint,
        ))
        .set_parent(world_entity);

    // Spawn the weapon using the chosen weapon type.
    commands
        .spawn((
            PlayerId(client_id),
            weapon_type.config_info(),
            SpawnBlueprint,
        ))
        .set_parent(world_entity);

    info!(
        "SERVER: Spawned player {} with spaceship: {:?} and weapon: {:?}",
        client_id, spaceship_type, weapon_type
    );
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
    mut evr_action: EventReader<MessageEvent<InputMessage<PlayerAction>>>,
    lobby_infos: Res<LobbyInfos>,
) {
    for event in evr_action.read() {
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

/// Stores the selected [`SpaceshipType`] of the client.
/// Defaults to [`SpaceshipType::default`] if no selection is found.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ClientSpaceshipSelection(HashMap<ClientId, SpaceshipType>);
