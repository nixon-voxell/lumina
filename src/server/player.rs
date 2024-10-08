use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use server::*;

use crate::protocol::INPUT_REPLICATION_GROUP;
use crate::shared::input::{MovementSet, PlayerAction};
use crate::shared::player::{
    shared_handle_player_movement, PlayerId, PlayerMovement, ReplicatePlayerBundle,
};

use super::lobby::{ClientInfos, Lobby};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                replicate_inputs.after(MainSet::EmitEvents),
                handle_input_spawn.in_set(ServerReplicationSet::ClientReplication),
            ),
        )
        .add_systems(
            FixedUpdate,
            handle_player_movement.in_set(MovementSet::Input),
        );
    }
}

/// Spawn an entity for a given client.
pub(super) fn spawn_player_entity(commands: &mut Commands, client_id: ClientId) -> Entity {
    info!("Spawn player for {:?}", client_id);

    let replicate = Replicate {
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
    };

    commands
        .spawn((
            ReplicatePlayerBundle::new(client_id, Position::default(), Rotation::default()),
            replicate,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(40.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

/// Adds input action entity to [`ClientInfo`] and replicate it back to other clients.
fn handle_input_spawn(
    mut commands: Commands,
    q_actions: Query<(&PlayerId, Entity), (Added<ActionState<PlayerAction>>, Added<Replicated>)>,
    mut client_infos: ResMut<ClientInfos>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (id, entity) in q_actions.iter() {
        let client_id = id.0;
        info!("Received input spawn from {client_id:?}");

        if let Some(info) = client_infos.get_mut(&client_id) {
            info.input = Some(entity);

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

/// Replicate the inputs of a client to other clients
/// so that a client can predict other clients.
fn replicate_inputs(
    q_lobbies: Query<&Lobby>,
    mut connection: ResMut<ConnectionManager>,
    mut input_events: EventReader<MessageEvent<InputMessage<PlayerAction>>>,
    client_infos: Res<ClientInfos>,
    // room_manager: Res<RoomManager>,
) {
    for event in input_events.read() {
        let inputs = event.message();
        let client_id = event.context();

        let Some(info) = client_infos.get(client_id) else {
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

fn handle_player_movement(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId)>,
    client_infos: Res<ClientInfos>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
) {
    for (action_state, id) in q_actions.iter() {
        let Some(player_entity) = client_infos.get(&id.0).map(|info| info.player) else {
            continue;
        };

        shared_handle_player_movement(action_state, player_entity, &mut player_movement_evw);
    }
}
