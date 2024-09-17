use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

use crate::game::player::{PlayerBundle, PlayerId, PlayerTransform};

use super::Connection;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>()
            .enable_state_scoped_entities::<LobbyState>()
            .init_resource::<MyLobbyId>()
            .add_systems(Update, (handle_predicted_spawn, handle_interpolated_spawn));
    }
}

fn handle_predicted_spawn(
    mut commands: Commands,
    q_predicted: Query<(Entity, &PlayerId), Added<Predicted>>,
) {
    for (entity, id) in q_predicted.iter() {
        info!("Spawn predicted entity.");

        commands.entity(entity).insert(PlayerBundle {
            id: *id,
            player_transform: PlayerTransform::default(),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    rect: Some(Rect::from_center_half_size(default(), Vec2::splat(20.0))),
                    ..default()
                },
                ..default()
            },
        });
    }
}

fn handle_interpolated_spawn(
    mut commands: Commands,
    interpolated: Query<(Entity, &PlayerId), Added<Interpolated>>,
) {
    for (entity, id) in interpolated.iter() {
        info!("Spawn interpolated entity.");

        commands.entity(entity).insert(PlayerBundle {
            id: *id,
            player_transform: PlayerTransform::default(),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: css::RED.into(),
                    rect: Some(Rect::from_center_half_size(default(), Vec2::splat(20.0))),
                    ..default()
                },
                transform: Transform::from_xyz(30.0, 0.0, 0.0),
                ..default()
            },
        });
    }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    Joined,
    InGame,
    Left,
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub(super) struct MyLobbyId(pub usize);
