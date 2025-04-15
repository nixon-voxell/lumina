use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::game::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::lobby::{ClientExitLobby, Lobby, LobbyInGame};

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
                    track_respawn_delays,
                ),
            )
            .observe(reset_all_spaceships_in_lobby)
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
            ));

            info!("Player {:?} will respawn after 5 seconds", player_id);
        }
    }
}

/// Process respawn delays and respawn players when timer finishes and resetting its position and health to the initial values
fn track_respawn_delays(
    time: Res<Time>,
    mut commands: Commands,
    mut q_respawn_delays: Query<(Entity, &mut RespawnDelay, &PlayerId)>,
    mut q_spaceships: Query<
        (
            &mut Position,
            &mut Rotation,
            &SpawnPointEntity,
            &MaxHealth,
            &mut Health,
        ),
        With<Spaceship>,
    >,
    q_global_transforms: Query<&GlobalTransform>,
    player_infos: Res<PlayerInfos>,
    mut q_weapons: Query<(&mut WeaponState, &Weapon), With<SourceEntity>>,
) {
    for (entity, mut respawn_delay, player_id) in q_respawn_delays.iter_mut() {
        respawn_delay.timer.tick(time.delta());

        if respawn_delay.timer.just_finished() {
            if let Ok((mut position, mut rotation, spawn_point_entity, max_health, mut health)) =
                q_spaceships.get_mut(entity)
            {
                let Ok((_, spawn_rotation, spawn_translation)) = q_global_transforms
                    .get(spawn_point_entity.0)
                    .map(|transform| transform.to_scale_rotation_translation())
                else {
                    warn!("No valid spawn point for entity {:?}", entity);
                    continue;
                };

                // Reset position and health.
                position.0 = spawn_translation.xy();
                *rotation = Rotation::radians(spawn_rotation.to_scaled_axis().z);
                **health = **max_health;

                // Reload weapon
                if let Some(weapon_entity) = player_infos[PlayerInfoType::Weapon].get(player_id) {
                    if let Ok((mut weapon_state, weapon)) = q_weapons.get_mut(*weapon_entity) {
                        weapon_state.reload(weapon);
                        debug!("Reloaded weapon for player_id {:?}", player_id);
                    } else {
                        warn!("Failed to reload weapon for player_id {:?}", player_id);
                    }
                }

                // Remove delay and Dead
                commands.entity(entity).remove::<(Dead, RespawnDelay)>();

                info!("Player {:?} has respawned after death penalty", entity);
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct ResetAllSpaceshipsInLobby;

/// Resets all spaceship health, energy, weapon within the lobby when a game starts.
fn reset_all_spaceships_in_lobby(
    trigger: Trigger<ResetAllSpaceshipsInLobby>,
    mut commands: Commands,
    q_lobbies: Query<&Lobby>,
    mut q_spaceships: Query<
        (
            &mut Health,
            &MaxHealth,
            &mut Energy,
            &Spaceship,
            &PlayerId,
            Entity,
        ),
        With<SourceEntity>,
    >,
    q_dash_cooldowns: Query<Entity, With<DashCooldown>>,
    player_infos: Res<PlayerInfos>,
    mut q_weapons: Query<(&mut WeaponState, &Weapon), With<SourceEntity>>,
) {
    let lobby_entity = trigger.entity();

    // Get the Lobby component for this specific room
    let Ok(lobby) = q_lobbies.get(lobby_entity) else {
        warn!("No lobby found for entity {:?}", lobby_entity);
        return;
    };

    info!("Resetting spaceships for lobby {:?}", lobby_entity);

    for client_id in lobby.iter() {
        if let Some(spaceship_entity) =
            player_infos[PlayerInfoType::Spaceship].get(&PlayerId(*client_id))
        {
            if let Ok((mut health, max_health, mut energy, spaceship, _player_id, entity)) =
                q_spaceships.get_mut(*spaceship_entity)
            {
                // Reset health
                **health = **max_health;

                // Reset energy
                energy.energy = spaceship.energy.max_energy;
                energy.cooldown = 0.0;

                // Remove dash cooldown if present
                if q_dash_cooldowns.contains(entity) {
                    commands.entity(entity).remove::<DashCooldown>();
                }

                // Remove Dead and RespawnDelay, ensure SpaceshipAction
                commands
                    .entity(entity)
                    .remove::<(Dead, RespawnDelay)>()
                    .insert(SpaceshipAction::default());

                // Reload weapon
                if let Some(weapon_entity) =
                    player_infos[PlayerInfoType::Weapon].get(&PlayerId(*client_id))
                {
                    if let Ok((mut weapon_state, weapon)) = q_weapons.get_mut(*weapon_entity) {
                        weapon_state.reload(weapon);
                    }
                }

                info!(
                    "Reset spaceship for client {}: health={}/{}, energy={}/{}",
                    client_id, **health, **max_health, energy.energy, spaceship.energy.max_energy
                );
            }
        }
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
