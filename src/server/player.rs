use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use server::*;

use crate::protocol::INPUT_REPLICATION_GROUP;
use crate::shared::input::PlayerAction;
use crate::shared::player::{PlayerId, PlayerInfos, SpaceShipType};

use super::lobby::Lobby;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                replicate_inputs.after(MainSet::EmitEvents),
                handle_input_spawn.in_set(ServerReplicationSet::ClientReplication),
            ),
        );
    }
}

/// Spawn an entity for a given client.
pub(super) fn spawn_player_entity(commands: &mut Commands, client_id: ClientId) -> Entity {
    info!("Spawn player for {:?}", client_id);

    let player_entity = commands
        .spawn((
            PlayerId(client_id),
            Replicate {
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
            },
        ))
        .id();

    commands
        .spawn((SpaceShipType::Assassin.ship_info(), SpawnBlueprint))
        .set_parent(player_entity);

    player_entity
}

/// Adds input action entity to [`PlayerInfos`] and replicate it back to other clients.
fn handle_input_spawn(
    mut commands: Commands,
    q_actions: Query<(&PlayerId, Entity), (Added<ActionState<PlayerAction>>, Added<Replicated>)>,
    player_infos: Res<PlayerInfos>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (id, entity) in q_actions.iter() {
        let client_id = id.0;
        info!("Received input spawn from {client_id:?}");

        if let Some(info) = player_infos.get(&client_id) {
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
            room_manager.add_entity(entity, info.room_id());
        }
    }
}

/// Replicate the inputs (actions) of a client to other clients
/// so that a client can predict other clients.
fn replicate_inputs(
    q_lobbies: Query<&Lobby>,
    mut connection: ResMut<ConnectionManager>,
    mut action_evr: EventReader<MessageEvent<InputMessage<PlayerAction>>>,
    player_infos: Res<PlayerInfos>,
) {
    for event in action_evr.read() {
        let inputs = event.message();
        let client_id = event.context();

        let Some(info) = player_infos.get(client_id) else {
            continue;
        };

        let Ok(lobby) = q_lobbies.get(info.lobby) else {
            continue;
        };

        // OPTIONAL: Do some validation on the inputs to check that there's no cheating

        // Rebroadcast the input to other clients inside the lobby.
        for client_id in lobby.iter().filter(|id| *id != client_id) {
            let _ = connection.send_message::<InputChannel, _>(*client_id, inputs);
        }
    }
}
