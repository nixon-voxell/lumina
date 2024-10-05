use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use server::*;
use smallvec::SmallVec;

use crate::protocol::{ExitLobby, LobbyStatus, Matchmake, ReliableChannel};
use crate::shared::input::PlayerAction;
use crate::shared::player::{
    shared_handle_player_movement, PlayerId, PlayerMovement, ReplicatePlayerBundle,
};
use crate::shared::MovementSet;
use crate::utils::EntityRoomId;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientInfos>()
            .add_event::<ClientExitLobby>()
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(
                Update,
                (
                    cleanup_empty_lobbies,
                    propagate_lobby_status,
                    handle_matchmaking,
                    handle_disconnection,
                    handle_exit_lobby,
                    handle_player_input_spawn,
                    execute_exit_lobby,
                ),
            )
            .add_systems(
                FixedUpdate,
                handle_player_movement.in_set(MovementSet::Input),
            );
    }
}

fn spawn_debug_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::NONE.into(),
                ..default()
            },
            ..default()
        },
    ));
}

fn cleanup_empty_lobbies(
    mut commands: Commands,
    q_lobbies: Query<(Entity, &Lobby), (Changed<Lobby>, Without<LobbyInGame>)>,
) {
    for (entity, lobby) in q_lobbies.iter() {
        if lobby.is_empty() {
            println!("Removing empty lobby: {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}

/// Send [`LobbyStatus`] message to clients on change.
fn propagate_lobby_status(
    q_lobbies: Query<(&Lobby, Entity), Changed<Lobby>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
) {
    for (lobby, lobby_entity) in q_lobbies.iter() {
        let client_count = lobby.len() as u8;
        let room_id = lobby_entity.room_id();

        // Send message to clients to notify about the changes.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &LobbyStatus {
                room_id,
                client_count,
            },
            room_id,
            &room_manager,
        );
    }
}

fn handle_matchmaking(
    mut commands: Commands,
    mut matchmake_evr: EventReader<MessageEvent<Matchmake>>,
    mut q_lobbies: Query<
        (&mut Lobby, &LobbySize, Entity),
        (Without<LobbyFull>, Without<LobbyInGame>),
    >,
    mut room_manager: ResMut<RoomManager>,
    mut client_infos: ResMut<ClientInfos>,
) {
    for matchmake in matchmake_evr.read() {
        let client_id = matchmake.context;

        // Already matchmake
        if client_infos.contains_key(&client_id) {
            warn!("Recieved duplicated matchmake commands from {client_id:?}");
            continue;
        }

        let lobby_size = *matchmake.message;
        let mut lobby_entity = None;

        // Find an available lobby to join.
        for (mut lobby, size, entity) in q_lobbies.iter_mut() {
            // Only find lobbies with the correct size.
            if lobby_size != **size {
                continue;
            }

            if lobby.len() < **size as usize {
                lobby.push(client_id);
                lobby_entity = Some(entity);

                if lobby.len() == **size as usize {
                    // Tag lobby as full so that this lobby won't show up the
                    // next time a new client requests to join. (optimization)
                    commands.entity(entity).insert(LobbyFull);
                }

                break;
            }
        }

        // If there is no available lobby to join, create a new one.
        let lobby_entity = lobby_entity
            .unwrap_or_else(|| commands.spawn(LobbyBundle::new(lobby_size, client_id)).id());

        let player_entity = spawn_player_entity(&mut commands, client_id);

        room_manager.add_client(client_id, lobby_entity.room_id());
        room_manager.add_entity(player_entity, lobby_entity.room_id());

        // Cache client info
        client_infos.insert(
            client_id,
            ClientInfo {
                lobby: lobby_entity,
                player: player_entity,
                input: None,
            },
        );
    }
}

fn handle_disconnection(
    mut commands: Commands,
    mut disconnect_evr: EventReader<DisconnectEvent>,
    mut client_infos: ResMut<ClientInfos>,
    mut client_exit_lobby_evw: EventWriter<ClientExitLobby>,
) {
    if disconnect_evr.is_empty() == false {
        client_exit_lobby_evw.send_batch(
            disconnect_evr
                .read()
                .map(|disconnect| ClientExitLobby(disconnect.client_id)),
        );
    }

    for event in disconnect_evr.read() {
        if let Some(info) = client_infos.remove(&event.client_id) {
            if let Some(entity_cmd) = info.input.map(|e| commands.entity(e)) {
                entity_cmd.despawn_recursive();
            }

            commands.entity(info.player).despawn_recursive();
        }
    }
}

fn handle_exit_lobby(
    mut exit_lobby_evr: EventReader<MessageEvent<ExitLobby>>,
    mut client_exit_lobby_evw: EventWriter<ClientExitLobby>,
) {
    if exit_lobby_evr.is_empty() == false {
        client_exit_lobby_evw.send_batch(
            exit_lobby_evr
                .read()
                .map(|exit| ClientExitLobby(exit.context)),
        );
    }
}

fn execute_exit_lobby(
    mut commands: Commands,
    mut client_exit_lobby_evr: EventReader<ClientExitLobby>,
    mut q_lobbies: Query<&mut Lobby>,
    mut room_manager: ResMut<RoomManager>,
    mut client_infos: ResMut<ClientInfos>,
) {
    for exit_client in client_exit_lobby_evr.read() {
        let client_id = exit_client.id();
        let Some(client_info) = client_infos.remove(&client_id) else {
            continue;
        };

        if let Ok(mut lobby) = q_lobbies.get_mut(client_info.lobby) {
            info!(
                "Client {client_id:?} exited lobby {:?}",
                client_info.room_id()
            );

            // Remove client from lobby and room.
            lobby.remove_client(&client_id);
            room_manager.remove_client(client_id, client_info.room_id());

            // Player might have already been despawned if it's a disconnection.
            if let Some(player) = commands.get_entity(client_info.player) {
                player.despawn_recursive();
                room_manager.remove_entity(client_info.player, client_info.room_id());
            }
            // Despawn input.
            if let Some(input) = client_info.input {
                commands.entity(input).despawn_recursive();
                room_manager.remove_entity(input, client_info.room_id());
            }

            // Now that someone left, the lobby is no longer full
            commands.entity(client_info.lobby).remove::<LobbyFull>();
        }
    }
}

/// Spawn an entity for a given client.
fn spawn_player_entity(commands: &mut Commands, client_id: ClientId) -> Entity {
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
            ReplicatePlayerBundle::new(client_id, Vec2::ZERO, 0.0),
            replicate,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(40.0)),
                    ..default()
                },
                // transform: Transform::from_scale(Vec3::splat(20.0)),
                ..default()
            },
        ))
        .id()
}

/// Adds input action entity to [`ClientInfo`].
fn handle_player_input_spawn(
    q_actions: Query<(&PlayerId, Entity), Added<ActionState<PlayerAction>>>,
    mut client_infos: ResMut<ClientInfos>,
) {
    for (id, entity) in q_actions.iter() {
        let client_id = id.0;
        if let Some(info) = client_infos.get_mut(&client_id) {
            info.input = Some(entity);
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

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub(super) struct ClientInfos(HashMap<ClientId, ClientInfo>);

#[derive(Debug)]
pub struct ClientInfo {
    pub lobby: Entity,
    pub player: Entity,
    pub input: Option<Entity>,
}

impl ClientInfo {
    /// Returns the [`RoomId`] of the lobby.
    pub fn room_id(&self) -> RoomId {
        self.lobby.room_id()
    }
}

#[derive(Bundle, Default)]
pub(super) struct LobbyBundle {
    pub lobby: Lobby,
    pub size: LobbySize,
}

impl LobbyBundle {
    pub fn new(size: u8, initial_client: ClientId) -> Self {
        Self {
            size: LobbySize(size),
            lobby: Lobby(SmallVec::from_slice(&[initial_client])),
        }
    }
}

/// A vec of clients currently inside the lobby.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Lobby(SmallVec<[ClientId; 6]>);

impl Lobby {
    pub fn remove_client(&mut self, client_id: &ClientId) {
        if let Some(index) = self.iter().position(|id| id == client_id) {
            self.swap_remove(index);
        }
    }
}

#[derive(Event)]
pub(super) struct ClientExitLobby(ClientId);

impl ClientExitLobby {
    pub fn id(&self) -> ClientId {
        self.0
    }
}

/// Size of lobby, indicating the max number of players in the lobby.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub(super) struct LobbySize(u8);

/// Tag for specifying a lobby is currently in game.
#[derive(Component, Default)]
pub(super) struct LobbyInGame;

/// Tag for specifying a lobby is currently full.
#[derive(Component, Default)]
pub(super) struct LobbyFull;
