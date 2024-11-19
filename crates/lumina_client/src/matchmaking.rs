use avian2d::prelude::*;
use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;

use super::player::LocalPlayerId;
use super::ui::{lobby::LobbyFunc, Screen};
use super::LocalClientId;

pub(super) struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Move this to InGame.
        app.init_resource::<TerrainEntity>();

        app.add_systems(OnEnter(Screen::Matchmaking), spawn_lobby)
            .add_systems(
                Update,
                handle_lobby_data.run_if(in_state(Screen::Matchmaking)),
            );
    }
}

fn spawn_lobby(mut _commands: Commands) {
    // commands.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
}

/// Digest data from [`LobbyData`].
///
/// When this message is sent, it means that we are officially in multiplayer mode.
fn handle_lobby_data(
    mut lobby_data_evr: EventReader<MessageEvent<LobbyData>>,
    mut lobby_func: ResMut<LobbyFunc>,
    mut next_screen_state: ResMut<NextState<Screen>>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
    terrain_entity: Res<TerrainEntity>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
) {
    for data in lobby_data_evr.read() {
        let data = data.message();
        // Update ui.
        lobby_func.room_id = Some(data.room_id.0);

        // Generate map.
        // TODO: Reuse terrain entity!
        generate_terrain_evw.send(GenerateTerrain {
            seed: data.seed,
            entity: **terrain_entity,
            layers: CollisionLayers::ALL,
        });

        // Update screen state.
        next_screen_state.set(Screen::MultiplayerLobby);
        // Set local player id to the networked version of player id.
        **local_player_id = PlayerId(**local_client_id);
    }
}

// TODO: Move this to InGame.
/// The one and only [`Entity`] that holds all the terrain data in the client.
///
/// See: [lumina_terrain].
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct TerrainEntity(pub Entity);

impl FromWorld for TerrainEntity {
    fn from_world(world: &mut World) -> Self {
        let entity = world.spawn_empty().id();
        Self(entity)
    }
}
