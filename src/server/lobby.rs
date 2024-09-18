use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use smallvec::SmallVec;

use crate::game::player::{PlayerBundle, PlayerId, PlayerTransform};
use crate::protocol::{ExitLobby, LobbyStatus, Matchmake, ReliableChannel};
use crate::utils::EntityRoomId;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientInfos>().add_systems(
            Update,
            (
                cleanup_empty_lobbies,
                propagate_lobby_status,
                handle_matchmaking,
                handle_exit_lobby,
            ),
        );
    }
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
    for (lobby, entity) in q_lobbies.iter() {
        let client_count = lobby.len() as u8;
        let room_id = entity.room_id();

        // Send message to clients to notify about the changes.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &LobbyStatus {
                entity,
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
            },
        );
    }
}

fn handle_exit_lobby(
    mut commands: Commands,
    mut exit_lobby_evt: EventReader<MessageEvent<ExitLobby>>,
    mut q_lobbies: Query<&mut Lobby>,
    mut room_manager: ResMut<RoomManager>,
    mut client_infos: ResMut<ClientInfos>,
) {
    for exit_lobby in exit_lobby_evt.read() {
        let client_id = exit_lobby.context;
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
            // Despawn player.
            commands.entity(client_info.player).despawn();
        }
    }
}

/// Spawn an entity for a given client.
fn spawn_player_entity(commands: &mut Commands, client_id: ClientId) -> Entity {
    info!("Spawn player for {:?}", client_id);
    let replicate = Replicate {
        sync: SyncTarget {
            prediction: NetworkTarget::Single(client_id),
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
            PlayerBundle {
                id: PlayerId(client_id),
                player_transform: PlayerTransform::default(),
                sprite_bundle: SpriteBundle::default(),
            },
            replicate,
        ))
        .id()
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct ClientInfos(HashMap<ClientId, ClientInfo>);

#[derive(Debug)]
pub struct ClientInfo {
    pub lobby: Entity,
    pub player: Entity,
}

impl ClientInfo {
    /// Returns the [`RoomId`] of the lobby.
    pub fn room_id(&self) -> RoomId {
        self.lobby.room_id()
    }
}

#[derive(Bundle, Default)]
pub struct LobbyBundle {
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

/// Size of lobby, indicating the max number of players in the lobby.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct LobbySize(u8);

/// Tag for specifying a lobby is currently in game.
#[derive(Component, Default)]
pub struct LobbyInGame;

/// Tag for specifying a lobby is currently full.
#[derive(Component, Default)]
pub struct LobbyFull;
