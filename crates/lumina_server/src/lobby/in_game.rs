use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::game::ResetSpaceships;
use crate::player::objective::{ObjectiveAreaManager, ResetObjectiveArea, OBJECTIVE_AREA_COUNT};

use super::{LobbyFull, LobbyInGame};

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_countdown, start_game, manage_objective_areas),
        );
    }
}

fn start_countdown(mut commands: Commands, q_lobbies: Query<Entity, Added<LobbyFull>>) {
    for entity in q_lobbies.iter() {
        // Initialize the countdown timer for 5 seconds if it's not already set
        commands
            .entity(entity)
            .insert(CountdownTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }
}

/// Manages the countdown and starts the game for each lobby individually
fn start_game(
    mut commands: Commands,
    mut q_lobbies: Query<(&mut CountdownTimer, Entity), With<LobbyFull>>,
    q_spaceships: Query<Entity, (With<Spaceship>, With<SourceEntity>, With<SpawnPointEntity>)>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    time: Res<Time>,
) {
    for (mut countdown_timer, entity) in q_lobbies.iter_mut() {
        // When the countdown reaches zero, start the game.
        if countdown_timer.tick(time.delta()).just_finished() {
            // Spawn map and send messages to notify clients.
            commands
                .spawn((MapType::AbandonedFactory.info(), SpawnBlueprint))
                .set_parent(entity);

            let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
                &StartGame,
                entity.room_id(),
                &room_manager,
            );

            // Trigger spaceship reset
            commands.trigger_targets(ResetSpaceships, entity);

            commands
                .entity(entity)
                .insert(LobbyInGame)
                // Remove the countdown timer after the game starts.
                .remove::<CountdownTimer>();

            for spaceship_entity in q_spaceships.iter() {
                println!("\n\nremove spaceship spawn point for {spaceship_entity}");
                commands
                    .entity(spaceship_entity)
                    .remove::<SpawnPointEntity>();
            }

            info!("Game started for lobby {entity}!");
        }
    }
}

// TODO: Needs better management, and only open when one area at a time...
fn manage_objective_areas(
    mut commands: Commands,
    // Manage sandbox managers only.
    mut q_manager: Query<&mut ObjectiveAreaManager, With<LobbyInGame>>,
    // Do no reset already resetting areas.
    q_areas: Query<(&ObjectiveArea, Has<ActiveObjectiveArea>), Without<ResetObjectiveArea>>,
) {
    for mut manager in q_manager
        .iter_mut()
        // TODO: Use a better marker to indicate that the objective area manager is ready.
        .filter(|m| m.areas.len() == OBJECTIVE_AREA_COUNT)
    {
        let area_entity = manager.areas[manager.selected_index];
        if let Ok((area, is_active)) = q_areas.get(area_entity) {
            let depleted = area.ores.unused().is_empty();
            if depleted == false {
                continue;
            }

            if is_active {
                // Active area has been depleted, choose a new active area!
                commands.entity(area_entity).remove::<ActiveObjectiveArea>();
                let mut new_index = rand::random::<u32>() as usize % manager.areas.len();
                if new_index == manager.selected_index {
                    new_index = (new_index + 1) % manager.areas.len();
                }

                manager.selected_index = new_index;
            } else {
                // Reset in 5 seconds.
                commands.entity(area_entity).insert((
                    ResetObjectiveArea(Timer::from_seconds(5.0, TimerMode::Once)),
                    ActiveObjectiveArea,
                ));
            }
        }
    }
}

/// Countdown before the game starts (in seconds).
#[derive(Component, Deref, DerefMut)]
pub struct CountdownTimer(Timer);

#[derive(Component)]
pub struct ActiveObjectiveArea;
