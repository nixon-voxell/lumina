use crate::effector::InteractedEffector;
use avian2d::prelude::Position;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::effector::MatchmakeEffector;
use lumina_shared::effector::TesseractEffector;
use lumina_shared::player::lumina::CollectedLuminas;
use lumina_shared::player::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use super::effector::effector_interaction;
use super::ui::Screen;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LuminaSpawnTimer(Timer::from_seconds(
            15.0,
            TimerMode::Repeating,
        )))
        .insert_resource(PlayerScores::default())
        .add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(OnExit(Screen::LocalLobby), despawn_lobby)
        .add_systems(Update, handle_lumina_spawn_timer)
        .add_systems(
            Update,
            matchmake_effector_trigger.run_if(effector_interaction::<MatchmakeEffector>),
        )
        .add_systems(
            Update,
            tesseract_effector_trigger.run_if(effector_interaction::<TesseractEffector>),
        );
    }
}

/// Spawn lobby scene.
fn spawn_lobby(
    mut commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    let lobby_scene = commands.spawn(LocalLobbyBundle::default()).id();
    commands
        .spawn((LobbyType::Local.info(), SpawnBlueprint))
        .set_parent(lobby_scene);

    // Tesseract
    commands
        .spawn((
            TesseractType::Tesseract.config_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    commands
        .spawn((
            TesseractType::Tesseract.visual_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    // Spaceship
    commands
        .spawn((
            SpaceshipType::Assassin.config_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    // Weapon
    commands
        .spawn((
            WeaponType::Cannon.config_info(),
            SpawnBlueprint,
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    // Action
    commands
        .spawn((
            InputManagerBundle::with_map(PlayerAction::input_map()),
            PlayerId::LOCAL,
        ))
        .set_parent(lobby_scene);

    **main_window_transparency = 1.0;
}

/// Despawn lobby scene.
fn despawn_lobby(mut commands: Commands, q_lobby: Query<Entity, With<LocalLobby>>) {
    let lobby = q_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

/// Action performed after the matchmake effector is being triggered.
fn matchmake_effector_trigger(mut _next_screen_state: ResMut<NextState<Screen>>) {
    // next_screen_state.set(Screen::Matchmaking);
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
    time: Res<Time>,
    mut timer: ResMut<LuminaSpawnTimer>,
    mut spawn_lumina_events: EventWriter<SpawnLumina>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_lumina_events.send(SpawnLumina {
            position: Position::from_xy(100.0, 100.0),
            lifetime: 300.0,
        });
    }
}

// TESTING PURPOSES
/// Action performed after the tesseract effector is being triggered.
fn tesseract_effector_trigger(
    mut commands: Commands,
    q_tesseract_effectors: Query<Entity, (With<TesseractEffector>, With<InteractedEffector>)>,
    mut effector_popup_func: ResMut<EffectorPopupFunc>, // Assuming UI popup
    mut collected_luminas: ResMut<CollectedLuminas>,
    mut player_scores: ResMut<PlayerScores>, // Add PlayerScores resource.
    _q_players: Query<&PlayerId>,
) {
    for effector_entity in q_tesseract_effectors.iter() {
        // Perform your logic for Tesseract interaction.
        info!("Tesseract effector triggered.");

        // Remove the `InteractedEffector` marker to allow retriggering.
        commands
            .entity(effector_entity)
            .remove::<InteractedEffector>();

        // Deposit Luminas for each player.
        for (player_id, pending_count) in collected_luminas.pending.drain() {
            // Add pending Luminas to the player's score.
            let score = player_scores.scores.entry(player_id).or_insert(0);
            *score += pending_count;

            println!(
                "Player {:?} deposited {} Luminas. Total score: {}",
                player_id, pending_count, *score
            );
        }

        // Optional: Reset UI or button functionality.
        effector_popup_func.clear(); // Clear any active popup or progress UI.
    }
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
#[derive(Resource, Default)]
pub struct PlayerScores {
    pub scores: HashMap<PlayerId, u32>, // Tracks scores for each player.
}
