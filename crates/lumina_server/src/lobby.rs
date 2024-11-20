use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;
use smallvec::SmallVec;

use crate::player::PlayerClient;

use super::LobbyInfos;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientExitLobby>()
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(
                Update,
                (
                    cleanup_empty_lobbies,
                    propagate_lobby_status,
                    spawn_multiplayer_lobby,
                    start_game,
                    handle_disconnections,
                    handle_exit_lobby,
                    execute_exit_lobby,
                ),
            )
            .add_systems(
                PreUpdate,
                handle_matchmaking.in_set(ServerReplicationSet::ClientReplication),
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
    q_lobbies: Query<(Entity, &Lobby), (Changed<Lobby>, Without<LobbyInGame>, Without<LobbyFull>)>,
    mut clear_terrain_evw: EventWriter<ClearTerrain>,
) {
    for (entity, lobby) in q_lobbies.iter() {
        if lobby.is_empty() {
            info!("Removing empty lobby: {entity:?}");
            clear_terrain_evw.send(ClearTerrain(entity));
            commands.entity(entity).despawn_recursive();
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

        // Send message to clients to notify about the changes.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &LobbyUpdate { client_count },
            entity.room_id(),
            &room_manager,
        );
    }
}

fn spawn_multiplayer_lobby(mut commands: Commands, q_lobbies: Query<Entity, Added<Lobby>>) {
    for entity in q_lobbies.iter() {
        commands
            .spawn((LobbyType::Multiplayer.info(), SpawnBlueprint))
            .set_parent(entity);
    }
}

/// Generate terain and start the game when lobby is full.
fn start_game(
    mut commands: Commands,
    q_lobbies: Query<(&LobbySeed, Entity), Added<LobbyFull>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
) {
    for (&LobbySeed(seed), entity) in q_lobbies.iter() {
        generate_terrain_evw.send(GenerateTerrain {
            seed,
            entity,
            // TODO: Use 1 -> 32 layers lol.
            layers: CollisionLayers::ALL,
        });

        // Send message to clients to notify that the game has started.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &StartGame,
            entity.room_id(),
            &room_manager,
        );

        commands.entity(entity).insert(LobbyInGame);
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
    mut lobby_infos: ResMut<LobbyInfos>,
) {
    for matchmake in matchmake_evr.read() {
        let client_id = matchmake.context;

        // Already matchmake, something is wrong...
        if lobby_infos.contains_key(&client_id) {
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
        let lobby_entity = lobby_entity.unwrap_or_else(|| {
            let seed = rand::random();
            let entity = commands
                .spawn(LobbyBundle::new(client_id, lobby_size, seed))
                .id();

            entity
        });

        // Spawn player.
        commands.spawn(PlayerClient {
            client_id,
            lobby_entity,
        });
        room_manager.add_client(client_id, lobby_entity.room_id());
        lobby_infos.insert(client_id, lobby_entity);
    }
}

fn handle_disconnections(
    mut disconnect_evr: EventReader<DisconnectEvent>,
    mut client_exit_lobby_evw: EventWriter<ClientExitLobby>,
) {
    if disconnect_evr.is_empty() == false {
        client_exit_lobby_evw.send_batch(
            disconnect_evr
                .read()
                .map(|disconnect| ClientExitLobby(disconnect.client_id)),
        );
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
    mut player_infos: ResMut<PlayerInfos>,
    mut lobby_infos: ResMut<LobbyInfos>,
) {
    for exit_client in client_exit_lobby_evr.read() {
        let client_id = exit_client.id();

        if let Some(lobby_entity) = lobby_infos.remove(&client_id) {
            let room_id = lobby_entity.room_id();
            // Remove client from the lobby.
            if let Ok(mut lobby) = q_lobbies.get_mut(lobby_entity) {
                lobby.remove_client(&client_id);
                // Now that someone left, the lobby is no longer full
                commands.entity(lobby_entity).remove::<LobbyFull>();
            }

            // Remove client from the room.
            room_manager.remove_client(client_id, room_id);

            // Despawn everything from the player.
            let player_entities = player_infos.remove_all(&PlayerId(client_id));
            for entity in player_entities.iter().filter_map(|e| *e) {
                if let Some(entity_cmd) = commands.get_entity(entity) {
                    entity_cmd.despawn_recursive();
                }

                room_manager.remove_entity(entity, room_id);
            }
        }
    }
}

#[derive(Bundle)]
pub(super) struct LobbyBundle {
    pub lobby: Lobby,
    pub size: LobbySize,
    pub seed: LobbySeed,
}

impl LobbyBundle {
    pub fn new(initial_client: ClientId, size: u8, seed: u32) -> Self {
        Self {
            size: LobbySize(size),
            lobby: Lobby(SmallVec::from_slice(&[initial_client])),
            seed: LobbySeed(seed),
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

#[derive(Component, Debug, Deref, DerefMut)]
pub(super) struct LobbySeed(pub u32);

#[derive(Component, Debug, Deref, DerefMut)]
pub(super) struct LobbySize(pub u8);

/// Tag for specifying a lobby is currently in game.
#[derive(Component, Default)]
pub(super) struct LobbyInGame;

/// Tag for specifying a lobby is currently full.
#[derive(Component, Default)]
pub(super) struct LobbyFull;

#[derive(Event)]
pub(super) struct ClientExitLobby(ClientId);

impl ClientExitLobby {
    pub fn id(&self) -> ClientId {
        self.0
    }
}
