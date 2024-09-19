use bevy::prelude::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::input::{PlayerAction, ReplicateInputBundle};
use crate::protocol::player::{PlayerId, PlayerTransform};
use crate::protocol::{LobbyStatus, PLAYER_REPLICATION_GROUP};

use super::{ui::lobby::LobbyFunc, Connection};

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>()
            .enable_state_scoped_entities::<LobbyState>()
            .init_resource::<MyLobbyId>()
            .add_systems(
                Update,
                (
                    handle_player_spawn,
                    handle_lobby_status_update.run_if(in_state(Connection::Connected)),
                ),
            );
    }
}

fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<
        (&PlayerId, Entity, Has<Predicted>),
        (
            Or<(Added<Predicted>, Added<Interpolated>)>,
            With<PlayerTransform>,
        ),
    >,
) {
    for (id, entity, is_predicted) in q_predicted.iter() {
        info!("Spawn predicted entity.");

        // Add visuals for player.
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: Some(Rect::from_center_half_size(default(), Vec2::splat(20.0))),
                ..default()
            },
            ..default()
        });

        if is_predicted {
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle {
                id: *id,
                replicate: Replicate {
                    group: PLAYER_REPLICATION_GROUP,
                    ..default()
                },
                input: InputManagerBundle::<PlayerAction> {
                    action_state: ActionState::default(),
                    input_map: PlayerAction::input_map(),
                },
                prepredicted: PrePredicted::default(),
            });
        }
    }
}

/// Update [`LobbyFunc`] and [`LobbyState`] based on [`LobbyStatus`].
fn handle_lobby_status_update(
    mut lobby_status_evr: EventReader<MessageEvent<LobbyStatus>>,
    mut lobby_func: ResMut<LobbyFunc>,
    lobby_state: Res<State<LobbyState>>,
    mut next_lobby_state: ResMut<NextState<LobbyState>>,
) {
    for lobby_status in lobby_status_evr.read() {
        let status = lobby_status.message();
        // Update ui
        lobby_func.curr_player_count = status.client_count;
        lobby_func.room_id = Some(status.room_id.0);

        // Update lobby state
        if *lobby_state != LobbyState::Joined {
            next_lobby_state.set(LobbyState::Joined);
        }
    }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    Joined,
    // InGame,
    // Left,
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub(super) struct MyLobbyId(pub usize);
