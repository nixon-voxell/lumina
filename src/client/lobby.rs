use bevy::color::palettes::css;
use bevy::prelude::*;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

use crate::game::player::PlayerBundle;
use crate::protocol::{PlayerId, PlayerTranslation};

use super::Connection;

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>()
            .enable_state_scoped_entities::<LobbyState>()
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
            player_translation: PlayerTranslation::default(),
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
    mut interpolated: Query<(Entity, &PlayerId), Added<Interpolated>>,
) {
    for (entity, id) in interpolated.iter_mut() {
        info!("Spawn interpolated entity.");
        commands.entity(entity).insert(PlayerBundle {
            id: *id,
            player_translation: PlayerTranslation::default(),
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

/// The client input only gets applied to predicted entities that we own
/// This works because we only predict the user's controlled entity.
/// If we were predicting more entities, we would have to only apply movement to the player owned one.
pub(crate) fn player_movement(
    mut position_query: Query<&mut PlayerTranslation, With<Predicted>>,
    // mut input_reader: EventReader<InputEvent<Inputs>>,
) {
    // for input in input_reader.read() {
    //     if let Some(input) = input.input() {
    //         for position in position_query.iter_mut() {
    //             // shared_movement_behaviour(position, input);
    //         }
    //     }
    // }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    Joined,
    Left,
}
