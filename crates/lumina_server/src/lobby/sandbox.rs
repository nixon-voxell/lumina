use bevy::{prelude::*, utils::HashMap};
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::player::PlayerClient;

pub(crate) struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SandboxWorlds(HashMap<ClientId, Entity>);

#[derive(Bundle, Default)]
struct SandboxBundle {
    pub world_id: PhysicsWorldId,
    pub spatial: SpatialBundle,
}

fn handle_enter_sandbox(
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
    mut worlds: ResMut<SandboxWorlds>,
    mut sandbox_evr: EventReader<MessageEvent<EnterSandbox>>,
) {
    for sandbox in sandbox_evr.read() {
        let client_id = sandbox.context;
        let seed = rand::random();
        let world_entity = commands
            .spawn(SandboxBundle {
                world_id: PhysicsWorldId(seed),
                ..default()
            })
            .id();
        worlds.insert(client_id, world_entity);

        // Spawn player.
        commands.spawn(PlayerClient {
            client_id,
            world_entity,
        });

        let _ = connection_manager.send_message_to_target::<ReliableChannel, _>(
            &EnterSandbox,
            NetworkTarget::Single(client_id),
        );
    }
}
