use bevy::prelude::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::{
    input::PlayerAction,
    player::{shared_movement_behaviour, PlayerTransform},
};

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerAction::input_map())
            .init_resource::<ActionState<PlayerAction>>()
            .insert_resource(PlayerAction::input_map())
            .add_systems(FixedUpdate, handle_player_input);
    }
}

fn handle_player_input(
    mut q_players: Query<&mut PlayerTransform, With<Predicted>>,
    action_state: Res<ActionState<PlayerAction>>,
) {
    let Ok(transform) = q_players.get_single_mut() else {
        return;
    };
    shared_movement_behaviour(transform, &action_state);
}
