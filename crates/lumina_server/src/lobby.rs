use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;
use smallvec::SmallVec;

mod in_game;
mod matchmaking;
mod sandbox;

use crate::player::{objective::ObjectiveAreaManager, ResetSpaceship};

use super::LobbyInfos;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            sandbox::SandboxPlugin,
            matchmaking::MatchmakingPlugin,
            in_game::InGamePlugin,
        ))
        .add_event::<ClientExitLobby>()
        .add_event::<LobbyRemoval>()
        .add_systems(
            Update,
            (
                cleanup_empty_lobbies,
                propagate_lobby_status,
                handle_disconnections,
                handle_exit_lobby,
                execute_exit_lobby,
            ),
        )
        .observe(reset_spaceships_in_lobby);
    }
}

fn cleanup_empty_lobbies(
    mut commands: Commands,
    q_lobbies: Query<(Entity, &Lobby), (Changed<Lobby>, Without<LobbyInGame>, Without<LobbyFull>)>,
    mut evw_lobby_removal: EventWriter<LobbyRemoval>,
) {
    for (entity, lobby) in q_lobbies.iter() {
        if lobby.is_empty() {
            info!("Removing empty lobby: {entity:?}");
            commands.entity(entity).despawn_recursive();
            evw_lobby_removal.send(LobbyRemoval(entity.room_id()));
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
        let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
            &LobbyUpdate { client_count },
            entity.room_id(),
            &room_manager,
        );
    }
}

fn handle_disconnections(
    mut evr_disconnect: EventReader<DisconnectEvent>,
    mut evw_client_exit_lobby: EventWriter<ClientExitLobby>,
) {
    if evr_disconnect.is_empty() == false {
        evw_client_exit_lobby.send_batch(
            evr_disconnect
                .read()
                .map(|disconnect| ClientExitLobby(disconnect.client_id)),
        );
    }
}

fn handle_exit_lobby(
    mut evr_exit_lobby: EventReader<MessageEvent<ExitLobby>>,
    mut evw_client_exit_lobby: EventWriter<ClientExitLobby>,
) {
    if evr_exit_lobby.is_empty() == false {
        evw_client_exit_lobby.send_batch(
            evr_exit_lobby
                .read()
                .map(|exit| ClientExitLobby(exit.context)),
        );
    }
}

fn execute_exit_lobby(
    mut commands: Commands,
    mut evr_client_exit_lobby: EventReader<ClientExitLobby>,
    mut q_lobbies: Query<&mut Lobby>,
    mut room_manager: ResMut<RoomManager>,
    mut player_infos: ResMut<PlayerInfos>,
    mut lobby_infos: ResMut<LobbyInfos>,
) {
    for exit_client in evr_client_exit_lobby.read() {
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

/// Trigger [`super::player::ResetSpaceship`] for all spaceship in the lobby.
fn reset_spaceships_in_lobby(
    trigger: Trigger<ResetSpaceshipsInLobby>,
    mut commands: Commands,
    q_lobbies: Query<&Lobby>,
    player_infos: Res<PlayerInfos>,
) {
    let lobby_entity = trigger.entity();

    // Get the Lobby component for this specific room
    let Ok(lobby) = q_lobbies.get(lobby_entity) else {
        warn!("No lobby found for entity {:?}", lobby_entity);
        return;
    };

    info!("Resetting spaceships for lobby {:?}", lobby_entity);

    let spaceship_entities = lobby
        .iter()
        .filter_map(|id| player_infos[PlayerInfoType::Spaceship].get(&PlayerId(*id)))
        .copied()
        .collect::<Vec<_>>();

    commands.trigger_targets(ResetSpaceship, spaceship_entities);
}

#[derive(Bundle)]
pub(super) struct LobbyBundle {
    pub lobby: Lobby,
    pub size: LobbySize,
    pub seed: LobbySeed,
    pub world_id: WorldIdx,
    pub spatial: SpatialBundle,
    pub objective_manager: ObjectiveAreaManager,
}

impl LobbyBundle {
    pub fn new(initial_client: ClientId, size: u8, seed: u32, world_entity: Entity) -> Self {
        Self {
            size: LobbySize(size),
            lobby: Lobby(SmallVec::from_slice(&[initial_client])),
            seed: LobbySeed(seed),
            world_id: WorldIdx::from_entity(world_entity),
            spatial: SpatialBundle::default(),
            objective_manager: ObjectiveAreaManager::default(),
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
pub struct LobbySeed(pub u32);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct LobbySize(pub u8);

/// Tag for specifying a lobby is currently in game.
#[derive(Component, Default)]
pub struct LobbyInGame;

/// Tag for specifying a lobby is currently full.
#[derive(Component, Default)]
pub struct LobbyFull;

/// Event sent when a client exits a lobby.
#[derive(Event)]
pub struct ClientExitLobby(pub ClientId);

impl ClientExitLobby {
    pub fn id(&self) -> ClientId {
        self.0
    }
}

/// Event sent when a lobby is being removed and despawned.
/// Use this to cleanup whatever cached lobby data.
#[derive(Event)]
pub struct LobbyRemoval(pub RoomId);

/// Trigger [`ResetSpaceship`] for all spaceship in the lobby.
#[derive(Event, Debug)]
pub struct ResetSpaceshipsInLobby;
