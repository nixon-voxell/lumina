use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;

use crate::protocol::{Matchmake, ReliableChannel};
use crate::shared::input::LocalInputBundle;
use crate::shared::player::LocalPlayerBundle;

use super::effector::effector_interaction;
use super::multiplayer_lobby::MatchmakeState;
use super::player::MyPlayer;
use super::ui::Screen;

pub(super) struct LocalLobbyPlugin;

impl Plugin for LocalLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::LocalLobby), init_lobby)
            .add_systems(OnExit(Screen::LocalLobby), despawn_lobby)
            .add_systems(
                Update,
                matchmake_effector.run_if(effector_interaction::<MatchmakeEffector>),
            );

        app.register_type::<MatchmakeEffector>()
            .register_type::<TutorialEffector>();
    }
}

/// Spawn lobby scene with player.
fn init_lobby(mut commands: Commands) {
    commands
        .spawn(LocalLobbySceneBundle::default())
        .with_children(|children| {
            children.spawn((BlueprintInfo::from_path("levels/Lobby.glb"), SpawnBlueprint));
            children
                .spawn(LocalPlayerBundle::new(
                    Position::default(),
                    Rotation::radians(std::f32::consts::FRAC_PI_2),
                ))
                .insert(LocalInputBundle::default())
                .insert(MyPlayer);
        });
}

/// Despawn local lobby scene
fn despawn_lobby(mut commands: Commands, q_local_lobby: Query<Entity, With<LocalLobbyScene>>) {
    // Despawn local lobby.
    let lobby = q_local_lobby.single();
    commands.entity(lobby).despawn_recursive();
}

fn matchmake_effector(
    mut connection_manager: ResMut<ConnectionManager>,
    mut next_matchmake_state: ResMut<NextState<MatchmakeState>>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    next_matchmake_state.set(MatchmakeState::Joining);
    next_screen_state.set(Screen::LoadingMultiplayer);

    // TODO: Support different player count modes.
    const PLAYER_COUNT: u8 = 2;
    let _ = connection_manager.send_message::<ReliableChannel, _>(&Matchmake(PLAYER_COUNT));
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
