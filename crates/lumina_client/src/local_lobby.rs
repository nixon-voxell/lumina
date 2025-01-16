use avian2d::prelude::Position;
use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::effector::TesseractEffector;
use lumina_shared::player::lumina::CollectedLuminas;
use lumina_shared::player::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use super::ui::Screen;
use crate::effector::InteractedEffector;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LuminaSpawnTimer(Timer::from_seconds(
            15.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(OnExit(Screen::LocalLobby), despawn_lobby)
        .add_systems(Update, handle_lumina_spawn_timer)
        .observe(tesseract_effector_trigger);
    }
}

/// Spawn lobby scene.
fn spawn_lobby(mut commands: Commands, mut transparency_evw: EventWriter<MainWindowTransparency>) {
    commands
        .spawn(LocalLobbyBundle::default())
        .with_children(|builder| {
            builder.spawn((LobbyType::Local.info(), SpawnBlueprint));

            // Tesseract
            builder.spawn((
                TesseractType::Tesseract.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
            ));

            // Spaceship
            builder.spawn((
                SpaceshipType::Assassin.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
            ));

            // Weapon
            builder.spawn((
                WeaponType::Cannon.config_info(),
                SpawnBlueprint,
                PlayerId::LOCAL,
            ));

            // Action
            builder.spawn((
                InputManagerBundle::with_map(PlayerAction::input_map()),
                PlayerId::LOCAL,
            ));
        });

    transparency_evw.send(MainWindowTransparency(1.0));
}

/// Despawn lobby scene.
fn despawn_lobby(mut commands: Commands, q_lobby: Query<Entity, With<LocalLobby>>) {
    let lobby = q_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, With<Predicted>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
struct LuminaSpawnTimer(Timer);

fn handle_lumina_spawn_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<LuminaSpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.trigger(SpawnLumina {
            position: Position::from_xy(100.0, 100.0),
            lifetime: 300.0,
        });
    }
}

// TESTING PURPOSES
/// Action performed after the tesseract effector is being triggered.
fn tesseract_effector_trigger(
    trigger: Trigger<TesseractEffector>,
    mut commands: Commands,
    mut connection_manager: ResMut<ConnectionManager>,
    // mut q_players: Query<(&PlayerId, &mut CollectedLuminas)>,
    // mut player_scores: ResMut<PlayerScores>,
) {
    let effector_entity = trigger.entity();
    info!(
        "Tesseract effector triggered by entity {:?}",
        effector_entity
    );

    let _ = connection_manager.send_message::<ReliableChannel, _>(&DepositLumina);

    // // Handle each player near the Tesseract.
    // for (player_id, mut collected_luminas) in q_players.iter_mut() {
    //     if collected_luminas.pending > 0 {
    //         // Transfer pending Luminas to the player's score.
    //         let score = player_scores.scores.entry(*player_id).or_insert(0);
    //         *score += collected_luminas.pending;

    //         println!(
    //             "Player {:?} deposited {} Luminas. Total score: {}",
    //             player_id, collected_luminas.pending, *score
    //         );

    //         // Reset the player's pending Luminas.
    //         collected_luminas.pending = 0;
    //     }
    // }

    // Clear interaction marker.
    commands
        .entity(effector_entity)
        .remove::<InteractedEffector>();
}

#[derive(Bundle)]
pub(super) struct LocalLobbyBundle {
    local_lobby: LocalLobby,
    spatial: SpatialBundle,
    source: SourceEntity,
    world_id: PhysicsWorldId,
}

impl Default for LocalLobbyBundle {
    fn default() -> Self {
        Self {
            local_lobby: LocalLobby,
            spatial: SpatialBundle::default(),
            source: SourceEntity,
            world_id: PhysicsWorldId::default(),
        }
    }
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct LocalLobby;

// TESTING PURPOSES
// #[derive(Resource, Default)]
// pub struct PlayerScores {
//     pub scores: HashMap<PlayerId, u32>, // Tracks scores for each player.
// }
