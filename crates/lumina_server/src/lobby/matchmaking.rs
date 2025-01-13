use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::{player::PlayerClient, LobbyInfos};

use super::{Lobby, LobbyBundle, LobbyFull, LobbyInGame, LobbySize};

pub(super) struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_matchmaking,
                spawn_multiplayer_lobby,
                despawn_multiplayer_lobby,
            ),
        );
    }
}

/// Find or create a lobby for new players to join.
fn handle_matchmaking(
    mut commands: Commands,
    mut matchmake_evr: EventReader<MessageEvent<Matchmake>>,
    mut q_lobbies: Query<
        (&mut Lobby, &LobbySize, Entity),
        (Without<LobbyFull>, Without<LobbyInGame>),
    >,
    mut room_manager: ResMut<RoomManager>,
    mut connection_manager: ResMut<ConnectionManager>,
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
            world_entity: lobby_entity,
        });

        let room_id = lobby_entity.room_id();
        let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
            &LobbyData { room_id },
            NetworkTarget::Single(client_id),
        );

        room_manager.add_client(client_id, lobby_entity.room_id());
        lobby_infos.insert(client_id, lobby_entity);
    }
}

// Spawn multiplayer lobby scene when a new lobby is added.
fn spawn_multiplayer_lobby(mut commands: Commands, q_lobbies: Query<Entity, Added<Lobby>>) {
    for entity in q_lobbies.iter() {
        let multiplayer_lobby = commands
            .spawn((LobbyType::Multiplayer.info(), SpawnBlueprint))
            .set_parent(entity)
            .id();

        commands
            .entity(entity)
            .insert(MultiplayerLobby(multiplayer_lobby));
    }
}

// Despawn multiplayer lobby when entering into a game.
fn despawn_multiplayer_lobby(
    mut commands: Commands,
    q_lobbies: Query<(&MultiplayerLobby, Entity), Added<LobbyInGame>>,
) {
    for (multiplayer_lobby, entity) in q_lobbies.iter() {
        commands.entity(**multiplayer_lobby).despawn_recursive();
        commands.entity(entity).remove::<MultiplayerLobby>();
    }
}

#[derive(Component, Deref)]
struct MultiplayerLobby(Entity);
