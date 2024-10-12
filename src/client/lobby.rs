use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;

use crate::protocol::{LobbyStatus, SeedMessage};
use crate::shared::procedural_map::grid_map::GenerateMapEvent;

use super::{ui::lobby::LobbyFunc, Connection};

pub(super) struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LobbyState>().add_systems(
            Update,
            (
                handle_lobby_status_update.run_if(in_state(Connection::Connected)),
                handle_seed_message.run_if(in_state(Connection::Connected)),
            ),
        );
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

/// This function gets a seed number from a message and uses it to create the game level.
///
/// The seed number makes sure that the game level looks the same for everyone.
/// When we get the seed number, we send an event to start making the level.
fn handle_seed_message(
    mut seed_message_event_reader: EventReader<MessageEvent<SeedMessage>>,
    mut generate_map_event_writer: EventWriter<GenerateMapEvent>,
) {
    for seed_message in seed_message_event_reader.read() {
        let seed = seed_message.message().0;
        generate_map_event_writer.send(GenerateMapEvent(seed));
    }
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[source(Connection = Connection::Connected)]
pub(super) enum LobbyState {
    #[default]
    None,
    Joining,
    Joined,
}
