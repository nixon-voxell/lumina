use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;
use smallvec::SmallVec;

pub mod in_game;
pub mod matchmaking;

use super::LobbyInfos;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((matchmaking::MatchmakingPlugin, in_game::InGamePlugin))
            .add_event::<ClientExitLobby>()
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(
                Update,
                (
                    cleanup_empty_lobbies,
                    propagate_lobby_status,
                    handle_disconnections,
                    handle_exit_lobby,
                    execute_exit_lobby,
                ),
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

/// Send [`LobbyUpdate`] message to clients on change.
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
    pub world_id: PhysicsWorldId,
    pub spatial: SpatialBundle,
}

impl LobbyBundle {
    pub fn new(initial_client: ClientId, size: u8, seed: u32) -> Self {
        Self {
            size: LobbySize(size),
            lobby: Lobby(SmallVec::from_slice(&[initial_client])),
            seed: LobbySeed(seed),
            world_id: PhysicsWorldId(seed),
            spatial: SpatialBundle::default(),
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
