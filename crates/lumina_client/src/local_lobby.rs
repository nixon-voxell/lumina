use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_coroutine::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::spaceship::{Spaceship, SpaceshipType};
use lumina_shared::player::weapon::WeaponType;
use lumina_shared::prelude::*;
use lumina_ui::main_window::WINDOW_FADE_DURATION;
use lumina_ui::prelude::*;

use super::effector::effector_interaction;
use super::ui::Screen;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::LocalLobby),
            (spawn_lobby, despawn_networked_inputs),
        )
        .add_systems(OnExit(Screen::LocalLobby), despawn_lobby)
        .add_systems(Update, init_spaceship.run_if(in_state(Screen::LocalLobby)))
        .add_systems(
            Update,
            matchmake_effector_trigger.run_if(effector_interaction::<MatchmakeEffector>),
        );

        app.register_type::<MatchmakeEffector>()
            .register_type::<TutorialEffector>();
    }
}

/// Spawn lobby scene.
fn spawn_lobby(
    mut commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    let lobby_scene = commands.spawn(LocalLobbyBundle::default()).id();
    commands
        .spawn((
            BlueprintInfo::from_path("levels/Lobby.glb"),
            SpawnBlueprint,
            HideUntilReady,
        ))
        .set_parent(lobby_scene);

    // Spaceship
    commands
        .spawn((SpaceshipType::Assassin.config_info(), SpawnBlueprint))
        .set_parent(lobby_scene);

    // Weapon
    commands
        .spawn((WeaponType::Cannon.config_info(), SpawnBlueprint))
        .set_parent(lobby_scene);

    // Action
    commands
        .spawn(InputManagerBundle::with_map(PlayerAction::input_map()))
        .set_parent(lobby_scene);

    **main_window_transparency = 1.0;
}

/// Rotate the spaceship to face forward.
fn init_spaceship(mut commands: Commands, q_spaceships: Query<Entity, Added<Spaceship>>) {
    for entity in q_spaceships.iter() {
        commands
            .entity(entity)
            .insert(Rotation::radians(std::f32::consts::FRAC_PI_2));
    }
}

/// Despawn local lobby scene
fn despawn_lobby(mut commands: Commands, q_local_lobby: Query<Entity, With<LocalLobby>>) {
    // Despawn local lobby.
    let lobby = q_local_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

/// Action performed after the matchmake effector is being triggered.
fn matchmake_effector_trigger(
    mut commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    // TODO: Support different player count modes.
    const PLAYER_COUNT: u8 = 2;

    commands.add(Coroutine::new(|| {
        let mut res = co_break();
        res.add_subroutines((
            wait(std::time::Duration::from_secs_f32(WINDOW_FADE_DURATION)),
            |mut connection_manager: ResMut<ConnectionManager>,
             mut next_screen_state: ResMut<NextState<Screen>>| {
                next_screen_state.set(Screen::Matchmaking);

                let _ =
                    connection_manager.send_message::<ReliableChannel, _>(&Matchmake(PLAYER_COUNT));
                co_break()
            },
        ));
        res
    }));

    **main_window_transparency = 0.0;
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct MatchmakeEffector;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct TutorialEffector;

#[derive(Bundle)]
pub(super) struct LocalLobbyBundle {
    local_lobby: LocalLobby,
    spatial: SpatialBundle,
    source: SourceEntity,
    id: PlayerId,
}

impl Default for LocalLobbyBundle {
    fn default() -> Self {
        Self {
            local_lobby: LocalLobby,
            spatial: SpatialBundle::default(),
            source: SourceEntity,
            id: PlayerId::LOCAL,
        }
    }
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct LocalLobby;
