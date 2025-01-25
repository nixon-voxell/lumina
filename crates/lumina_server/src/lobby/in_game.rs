use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbySeed};

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_countdown, start_game));
    }
}

fn start_countdown(mut commands: Commands, q_lobbies: Query<Entity, Added<LobbyFull>>) {
    for entity in q_lobbies.iter() {
        // Initialize the countdown timer for 5 seconds if it's not already set
        commands.entity(entity).insert(CountdownTimer(5.0));
    }
}

/// Manages the countdown and starts the game for each lobby individually
fn start_game(
    mut commands: Commands,
    mut q_lobbies: Query<(&mut CountdownTimer, &LobbySeed, Entity), With<LobbyFull>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
    time: Res<Time>,
) {
    for (mut countdown_timer, &LobbySeed(seed), entity) in q_lobbies.iter_mut() {
        // Decrease the timer by the actual time elapsed (in seconds)
        countdown_timer.0 -= time.delta_seconds();

        // When the countdown reaches zero, start the game.
        if countdown_timer.0 <= 0.0 {
            // Generate terrain and send messages to notify clients.
            generate_terrain_evw.send(GenerateTerrain {
                seed,
                entity,
                layers: CollisionLayers::ALL,
                world_id: WorldIdx::from_entity(entity),
            });

            let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
                &StartGame { seed },
                entity.room_id(),
                &room_manager,
            );

            commands
                .entity(entity)
                .insert(LobbyInGame)
                // Remove the countdown timer after the game starts.
                .remove::<CountdownTimer>();

            info!("Game started for lobby {:?} with seed {:?}", entity, seed);
        }
    }
}

/// Countdown before the game starts (in seconds).
#[derive(Component)]
pub struct CountdownTimer(pub f32);
