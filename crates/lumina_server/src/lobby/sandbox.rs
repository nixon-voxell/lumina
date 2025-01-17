use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::player::SpawnClientPlayer;
use crate::LobbyInfos;

pub(crate) struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_enter_sandbox);
    }
}

fn handle_enter_sandbox(
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
    mut room_manager: ResMut<RoomManager>,
    mut lobbies: ResMut<LobbyInfos>,
    mut sandbox_evr: EventReader<MessageEvent<EnterSandbox>>,
) {
    for sandbox in sandbox_evr.read() {
        let client_id = sandbox.context;
        let world_entity = commands.spawn_empty().id();

        commands
            .entity(world_entity)
            .insert(SandboxBundle {
                world_id: PhysicsWorldId(world_entity.index()),
                ..default()
            })
            .with_children(|builder| {
                // Spawn the sandbox level.
                builder.spawn((LobbyType::Sandbox.info(), SpawnBlueprint));
            });

        // Spawn player.
        commands.trigger(SpawnClientPlayer {
            client_id,
            world_entity,
        });

        let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
            &EnterSandbox,
            NetworkTarget::Single(client_id),
        );

        room_manager.add_client(client_id, world_entity.room_id());
        // Add to the lobby hash map.
        lobbies.insert(client_id, world_entity);
    }
}

fn handle_lumina_spawn_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    if let Some(timer) = &mut *timer {
        if timer.tick(time.delta()).just_finished() {
            commands.trigger(SpawnLumina {
                position: Position::from_xy(100.0, 100.0),
                lifetime: 300.0,
            });
        }
    } else {
        *timer = Some(Timer::from_seconds(15.0, TimerMode::Repeating));
    }
}

#[derive(Bundle, Default)]
struct SandboxBundle {
    pub world_id: PhysicsWorldId,
    pub spatial: SpatialBundle,
}
