use bevy::prelude::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::{
    input::{PlayerAction, ReplicateInputBundle},
    player::{PlayerId, PlayerTransform},
    PLAYER_REPLICATION_GROUP,
};

use super::Connection;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>()
            .enable_state_scoped_entities::<LobbyState>()
            .init_resource::<MyLobbyId>()
            .add_systems(Update, handle_player_spawn);
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

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    // Joined,
    // InGame,
    // Left,
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub(super) struct MyLobbyId(pub usize);
