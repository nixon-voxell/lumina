use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use super::{LobbyFull, LobbyInGame, LobbyInfos, LobbySeed, TeamType};
use lumina_terrain::map::TerrainStates;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TeamToggle::new())
            .add_systems(Update, (start_game, init_spaceship_position));
    }
}

/// Generate terain and start the game when lobby is full.
fn start_game(
    mut commands: Commands,
    q_lobbies: Query<(&LobbySeed, Entity), Added<LobbyFull>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut generate_terrain_evw: EventWriter<GenerateTerrain>,
) {
    for (&LobbySeed(seed), entity) in q_lobbies.iter() {
        generate_terrain_evw.send(GenerateTerrain {
            seed,
            entity,
            layers: CollisionLayers::ALL,
            world_id: PhysicsWorldId(seed),
        });

        // Send message to clients to notify that the game has started.
        let _ = connection_manager.send_message_to_room::<ReliableChannel, _>(
            &StartGame { seed },
            entity.room_id(),
            &room_manager,
        );

        commands.entity(entity).insert(LobbyInGame);
    }
}

/// This function updates the position of newly spawned spaceships
fn init_spaceship_position(
    mut commands: Commands,
    mut q_spaceships: Query<
        (&mut Position, &mut Rotation, &PlayerId, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<PositionInitialized>,
        ),
    >,
    q_in_game_lobbies: Query<(), With<LobbyInGame>>,
    terrain_config: TerrainConfig,
    lobby_infos: Res<LobbyInfos>,
    mut team_toggle: ResMut<TeamToggle>, // Access the team toggle resource
) {
    let Some(terrain_config) = terrain_config.get() else {
        eprintln!("Terrain config is not available!");
        return;
    };

    let (bottom_left, upper_right) =
        TerrainStates::get_map_corners_without_noise_surr(terrain_config);

    println!(
        "\n\n\n\nDebugging Map Corners: Bottom-left: {:?}, Upper-right: {:?}",
        bottom_left, upper_right
    );

    for (mut position, _rotation, id, entity) in q_spaceships.iter_mut() {
        if lobby_infos
            .get(&**id)
            .is_some_and(|e| q_in_game_lobbies.contains(*e))
            == false
        {
            continue;
        }

        let (spawn_position, team_type) = if team_toggle.toggle {
            (bottom_left, TeamType::A)
        } else {
            (upper_right, TeamType::B)
        };

        team_toggle.toggle = !team_toggle.toggle; // Toggle the team for the next spawn

        *position = Position(spawn_position);
        commands
            .entity(entity)
            .insert((PositionInitialized, team_type));
    }
}

#[derive(Component)]
pub struct PositionInitialized;

#[derive(Resource)]
pub struct TeamToggle {
    pub toggle: bool,
}

impl TeamToggle {
    pub fn new() -> Self {
        Self { toggle: true }
    }
}
