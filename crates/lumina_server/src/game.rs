use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_terrain::prelude::*;
use server::*;

use crate::{
    lobby::{ClientExitLobby, Lobby, LobbyInGame},
    LobbyInfos,
};

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_spaceship_positions,
                respawn_spaceships,
                init_game,
                track_game_score,
                track_game_timer,
            ),
        )
        .observe(end_game);
    }
}

fn end_game(
    trigger: Trigger<EndGame>,
    mut commands: Commands,
    q_lobbies: Query<&Lobby>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
    mut evw_client_exit: EventWriter<ClientExitLobby>,
) {
    let entity = trigger.entity();

    // Entity must be a lobby entity for logic beneath to work.
    let Ok(lobby) = q_lobbies.get(entity) else {
        return;
    };

    for id in lobby.iter() {
        evw_client_exit.send(ClientExitLobby(*id));
    }

    let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
        &EndGame,
        entity.room_id(),
        &room_manager,
    );

    commands.entity(entity).remove::<LobbyInGame>();
}

/// Respawn the spaceship by resetting its position and health to the initial values
fn respawn_spaceships(
    mut q_spaceships: Query<
        (
            &Visibility,
            &mut Position,
            &InitPosition,
            &MaxHealth,
            &mut Health,
            &TeamType,
            &PlayerId,
        ),
        (With<Spaceship>, Changed<Visibility>, With<SourceEntity>),
    >,
    mut q_game_scores: Query<&mut GameScore>,
    lobby_infos: Res<LobbyInfos>,
) {
    // Spaceship becomes Visibility::Hidden when health drops to 0.
    for (_, mut position, position_initialized, max_health, mut health, team_type, id) in
        q_spaceships
            .iter_mut()
            .filter(|(viz, ..)| *viz == Visibility::Hidden)
    {
        if let Some(mut game_score) = lobby_infos
            .get(&**id)
            .and_then(|e| q_game_scores.get_mut(*e).ok())
        {
            game_score.scores[team_type.invert() as usize] += 1;
        }

        // Reset position and health.
        position.0 = position_initialized.0;
        **health = **max_health;
    }
}

/// Initialize [`GameScore`] and [`GameTimer`].
fn init_game(mut commands: Commands, q_lobbies: Query<Entity, Added<LobbyInGame>>) {
    for entity in q_lobbies.iter() {
        commands.entity(entity).insert((
            GameScore::new(15),
            GameTimer(Timer::from_seconds(60.0 * 2.5, TimerMode::Once)),
        ));
    }
}

/// Track game score and end game when either one of the team wins.
fn track_game_score(
    mut commands: Commands,
    q_game_scores: Query<(&GameScore, Entity), (Changed<GameScore>, With<LobbyInGame>)>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
) {
    for (game_score, entity) in q_game_scores.iter() {
        let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
            game_score,
            entity.room_id(),
            &room_manager,
        );

        if game_score
            .scores
            .iter()
            .any(|&score| score >= game_score.max_score)
        {
            commands.trigger_targets(EndGame, entity);
        }
    }
}

/// Track game timer and end game when timer reaches zero.
fn track_game_timer(
    mut commands: Commands,
    mut q_game_timers: Query<(&mut GameTimer, Entity), With<LobbyInGame>>,
    time: Res<Time>,
) {
    for (mut game_timer, entity) in q_game_timers.iter_mut() {
        if game_timer.tick(time.delta()).just_finished() {
            commands.trigger_targets(EndGame, entity);
        }
    }
}

// TODO: Utilize spawn points instead!
/// Updates the position of newly spawned spaceships based on their assigned team type.
fn init_spaceship_positions(
    mut commands: Commands,
    mut q_spaceship: Query<
        (&mut Position, &TeamType, &PlayerId, Entity),
        (With<Spaceship>, With<SourceEntity>, Without<InitPosition>),
    >,
    q_in_game_lobbies: Query<(), With<LobbyInGame>>,
    terrain_config: TerrainConfig,
    lobby_infos: Res<LobbyInfos>,
) {
    // Ensure the terrain config is available
    let Some(terrain_config) = terrain_config.get() else {
        return;
    };

    // Retrieve map corners (bottom-left and upper-right) based on terrain configuration
    let (bottom_left, upper_right) =
        TerrainStates::get_map_corners_without_noise_surr(terrain_config);

    for (mut spaceship_position, team_type, id, spaceship_entity) in q_spaceship.iter_mut() {
        // Skip if the spaceship is not part of an in-game lobby
        if lobby_infos
            .get(&**id)
            .is_some_and(|lobby_entity| q_in_game_lobbies.contains(*lobby_entity))
            == false
        {
            continue;
        }

        // Determine the position based on team type
        let new_position = match team_type {
            TeamType::A => Vec2::new(bottom_left.x, bottom_left.y),
            TeamType::B => Vec2::new(upper_right.x, upper_right.y),
        };

        *spaceship_position = Position(new_position);

        // Store the initial position in the PositionInitialized component
        commands
            .entity(spaceship_entity)
            .insert(InitPosition(new_position));
    }
}

/// Initial spawn position of the spaceships.
#[derive(Component)]
pub struct InitPosition(pub Vec2);

/// Time left for a game (in seconds).
#[derive(Component, Deref, DerefMut)]
pub struct GameTimer(Timer);
