use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_coroutine::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;

use crate::protocol::{Matchmake, ReliableChannel};
use crate::shared::input::{InputTarget, LocalInputBundle};
use crate::shared::player::{LocalPlayerBundle, SpaceShip, SpaceShipType};
use crate::ui::main_window::{MainWindowTransparency, WINDOW_FADE_DURATION};

use super::effector::effector_interaction;
use super::ui::Screen;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::LocalLobby), init_lobby)
            .add_systems(OnExit(Screen::LocalLobby), despawn_lobby)
            .add_systems(Update, init_input.run_if(in_state(Screen::LocalLobby)))
            .add_systems(
                Update,
                matchmake_effector_trigger.run_if(effector_interaction::<MatchmakeEffector>),
            );

        app.register_type::<MatchmakeEffector>()
            .register_type::<TutorialEffector>();
    }
}

/// Spawn lobby scene with player.
fn init_lobby(
    mut commands: Commands,
    mut main_window_transparency: ResMut<MainWindowTransparency>,
) {
    let lobby_scene = commands.spawn(LocalLobbySceneBundle::default()).id();
    commands
        .spawn((
            BlueprintInfo::from_path("levels/Lobby.glb"),
            SpawnBlueprint,
            HideUntilReady,
        ))
        .set_parent(lobby_scene);

    commands
        .spawn((
            SpaceShipType::Assassin.ship_info(),
            SpawnBlueprint,
            HideUntilReady,
        ))
        .set_parent(lobby_scene);

    **main_window_transparency = 1.0;
}

fn init_input(mut commands: Commands, q_player: Query<Entity, Added<SpaceShip>>) {
    let Ok(player_entity) = q_player.get_single() else {
        return;
    };

    commands
        .entity(player_entity)
        .insert((
            LocalPlayerBundle::default(),
            Rotation::radians(std::f32::consts::FRAC_PI_2),
        ))
        .insert(LocalInputBundle::new(InputTarget::new(player_entity)));
}

/// Despawn local lobby scene
fn despawn_lobby(mut commands: Commands, q_local_lobby: Query<Entity, With<LocalLobbyScene>>) {
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct MatchmakeEffector;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct TutorialEffector;

#[derive(Bundle, Default)]
pub(super) struct LocalLobbySceneBundle {
    local_lobby: LocalLobbyScene,
    spatial: SpatialBundle,
}

#[derive(Component, Default)]
/// Tag for the parent entity of the lobby scene.
pub(super) struct LocalLobbyScene;
