use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::{player::PlayerClient, LobbyInfos};

pub(crate) struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_enter_sandbox);
    }
}

#[derive(Bundle, Default)]
struct SandboxBundle {
    pub world_id: PhysicsWorldId,
    pub spatial: SpatialBundle,
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

        commands.entity(world_entity).insert(SandboxBundle {
            world_id: PhysicsWorldId(world_entity.index()),
            ..default()
        });

        // Spawn player.
        commands.spawn(PlayerClient {
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
