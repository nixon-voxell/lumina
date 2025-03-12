use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;
use smallvec::SmallVec;

use crate::player::objective::{ObjectiveAreaManager, ResetObjectiveArea};
use crate::player::SpawnClientPlayer;
use crate::LobbyInfos;

use super::Lobby;

pub(crate) struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_enter_sandbox, manage_sandbox_areas));
    }
}

fn handle_enter_sandbox(
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
    mut room_manager: ResMut<RoomManager>,
    mut lobbies: ResMut<LobbyInfos>,
    mut evr_sandbox: EventReader<MessageEvent<EnterSandbox>>,
) {
    for sandbox in evr_sandbox.read() {
        let client_id = sandbox.context;
        let world_entity = commands.spawn_empty().id();

        commands
            .entity(world_entity)
            .insert((
                SandboxBundle {
                    world_id: WorldIdx::from_entity(world_entity),
                    lobby: Lobby(SmallVec::from_slice(&[client_id])),
                    ..default()
                },
                // Game score for the score bar to appear and interact with deposition.
                GameScore::new(50),
            ))
            .with_children(|builder| {
                // Spawn the sandbox level.
                builder.spawn((MapType::Sandbox.info(), SpawnBlueprint));
            });

        // Spawn player.
        commands.trigger(SpawnClientPlayer {
            client_id,
            world_entity,
        });

        let _ = connection_manager.send_message_to_target::<OrdReliableChannel, _>(
            &EnterSandbox,
            NetworkTarget::Single(client_id),
        );

        // Send `LobbyData` with a generated room ID
        let room_id = world_entity.room_id();

        let _ = connection_manager.send_message_to_target::<OrdReliableChannel, _>(
            &LobbyData { room_id },
            NetworkTarget::Single(client_id),
        );

        room_manager.add_client(client_id, world_entity.room_id());
        // Add to the lobby hash map.
        lobbies.insert(client_id, world_entity);
    }
}

/// Reset sandbox objectives 5 seconds after they are depleted.
fn manage_sandbox_areas(
    mut commands: Commands,
    // Manage sandbox managers only.
    q_manager: Query<&ObjectiveAreaManager, With<Sandbox>>,
    // Do no reset already resetting areas.
    q_areas: Query<&ObjectiveArea, Without<ResetObjectiveArea>>,
) {
    for manager in q_manager.iter() {
        for area_entity in manager.areas.iter() {
            if let Ok(area) = q_areas.get(*area_entity) {
                let depleted = area.ores.unused().is_empty();

                if depleted {
                    // Reset in 5 seconds.
                    commands
                        .entity(*area_entity)
                        .insert(ResetObjectiveArea(Timer::from_seconds(
                            5.0,
                            TimerMode::Once,
                        )));
                }
            }
        }
    }
}

#[derive(Component, Default)]
pub(super) struct Sandbox;

#[derive(Bundle, Default)]
struct SandboxBundle {
    pub sandbox: Sandbox,
    pub world_id: WorldIdx,
    pub spatial: SpatialBundle,
    pub objective_manager: ObjectiveAreaManager,
    /// Just as a marker so that it gets cleanup on empty.
    pub lobby: Lobby,
}
