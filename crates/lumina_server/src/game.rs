use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
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
                    respawn_spaceships,
                    init_game,
                    propagate_game_score,
                    track_game_score,
                    track_game_timer,
                ),
            )
            .observe(reset_spaceships)
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
    mut commands: Commands,
    mut q_spaceships: Query<
        (
            &Visibility,
            &mut Position,
            &mut Rotation,
            &SpawnPointEntity,
            &MaxHealth,
            &mut Health,
            &CollectedLumina,
            &PlayerId,
            Entity,
        ),
        (With<Spaceship>, Changed<Visibility>, With<SourceEntity>),
    >,
    q_global_transforms: Query<&GlobalTransform>,
) {
    // Spaceship becomes Visibility::Hidden when health drops to 0.
    for (
        visibility,
        mut position,
        mut rotation,
        spawn_point_entity,
        max_health,
        mut health,
        collected_lumina,
        player_id,
        entity,
    ) in q_spaceships.iter_mut()
    {
        if *visibility == Visibility::Hidden {
            if collected_lumina.0 > 0 {
                info!(
                    "Player {:?} has died with {} collected lumina",
                    player_id, collected_lumina.0
                );
                commands.trigger(PlayerDeath {
                    player_entity: entity,
                    position: *position,
                });
            }

            let Ok((_, spawn_rotation, spawn_translation)) = q_global_transforms
                .get(spawn_point_entity.0)
                .map(|transform| transform.to_scale_rotation_translation())
            else {
                return;
            };

            // Reset position and health.
            position.0 = spawn_translation.xy();
            *rotation = Rotation::radians(spawn_rotation.to_scaled_axis().z);
            **health = **max_health;
        }
    }
}

#[derive(Event, Debug)]
pub struct ResetSpaceships;

/// Resets all spaceship health, energy, weapon  when a game starts
fn reset_spaceships(
    trigger: Trigger<ResetSpaceships>,
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
            GameTimer(Timer::from_seconds(60.0 * 4.0, TimerMode::Once)),
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
    pub player_entity: Entity,
    pub position: Position,
}
