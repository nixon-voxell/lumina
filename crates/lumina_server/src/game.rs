use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::game::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::lobby::{ClientExitLobby, Lobby, LobbyInGame};
use crate::player::ResetSpaceship;

mod teleporter;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(teleporter::TeleporterPlugin)
            .add_systems(
                Update,
                (
                    handle_player_death,
                    init_game,
                    propagate_game_score,
                    track_game_score,
                    track_game_timer,
                    track_respawn_delay,
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

/// Trigger PlayerDeath event, mark player as dead and set respawn delay.
fn handle_player_death(
    mut commands: Commands,
    mut q_spaceships: Query<
        (&Health, &Position, &PlayerId, Entity),
        (With<Spaceship>, With<SourceEntity>, Changed<Health>),
    >,
) {
    for (health, position, player_id, entity) in q_spaceships.iter_mut() {
        if **health <= 0.0 {
            commands.trigger_targets(
                PlayerDeath {
                    position: *position,
                },
                entity,
            );

            // Mark as dead and clear SpaceshipAction
            commands.entity(entity).insert((
                Dead,
                RespawnDelay {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                CancelAbility,
            ));

            info!("Player {:?} will respawn after 5 seconds", player_id);
        }
    }
}

/// Track [`RespawnDelay`] and respawn players when timer finishes
/// by resetting its [`Position`] & [`Rotation`] and triggering [`ResetSpaceship`].
fn track_respawn_delay(
    time: Res<Time>,
    mut commands: Commands,
    mut q_respawn_delays: Query<(
        &mut RespawnDelay,
        &mut Position,
        &mut Rotation,
        &SpawnPointEntity,
        Entity,
    )>,
    q_global_transforms: Query<&GlobalTransform>,
) {
    for (mut respawn_delay, mut position, mut rotation, spawn_point_entity, entity) in
        q_respawn_delays.iter_mut()
    {
        if respawn_delay.timer.tick(time.delta()).just_finished() == false {
            continue;
        }

        commands.entity(entity).remove::<(Dead, RespawnDelay)>();
        commands.trigger_targets(ResetSpaceship, entity);

        let Ok((_, spawn_rotation, spawn_translation)) = q_global_transforms
            .get(spawn_point_entity.0)
            .map(|transform| transform.to_scale_rotation_translation())
        else {
            warn!("No valid spawn point for entity {:?}", entity);
            continue;
        };

        // Reset position and rotation.
        position.0 = spawn_translation.xy();
        *rotation = Rotation::radians(spawn_rotation.to_scaled_axis().z);
    }
}

/// Initialize [`GameScore`] and [`GameTimer`].
fn init_game(mut commands: Commands, q_lobbies: Query<Entity, Added<LobbyInGame>>) {
    for entity in q_lobbies.iter() {
        commands.entity(entity).insert((
            GameScore::new(50),
            GameTimer(Timer::from_seconds(GAME_DURATION, TimerMode::Once)),
        ));
    }
}

/// Propagate game score to the clients.
fn propagate_game_score(
    q_game_scores: Query<(&GameScore, Entity), Changed<GameScore>>,
    mut connection_manager: ResMut<ConnectionManager>,
    room_manager: Res<RoomManager>,
) {
    for (game_score, entity) in q_game_scores.iter() {
        let _ = connection_manager.send_message_to_room::<OrdReliableChannel, _>(
            game_score,
            entity.room_id(),
            &room_manager,
        );
    }
}

/// Track game score and end game when either one of the team wins.
fn track_game_score(
    mut commands: Commands,
    q_game_scores: Query<(&GameScore, Entity), (Changed<GameScore>, With<LobbyInGame>)>,
) {
    for (game_score, entity) in q_game_scores.iter() {
        if game_score.score == game_score.max_score || game_score.score == 0 {
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

/// Time left for a game (in seconds).
#[derive(Component, Deref, DerefMut)]
pub struct GameTimer(Timer);

/// Triggered when a player dies.
#[derive(Event)]
pub struct PlayerDeath {
    /// The position when the player dies.
    pub position: Position,
}

/// Component to track the respawn delay
#[derive(Component)]
pub struct RespawnDelay {
    pub timer: Timer,
}
