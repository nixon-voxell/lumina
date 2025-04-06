use bevy::prelude::*;
use bevy_coroutine::prelude::*;
use bevy_motiongfx::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;

use crate::effector::{InteractedEffector, MatchmakeEffector};
use crate::typ_animation::AnimateTypAppExt;

use super::lobby::LobbyFunc;
use super::Screen;

const SANDBOX_BTN: &str = "btn:sandbox";
const MATCHMAKE_BTNS: &[&str] = &["btn:1v1", "btn:2v2", "btn:3v3"];
const CANCEL_BTN: &str = "btn:cancel-matchmake";

pub(super) struct GameModeUiPlugin;

impl Plugin for GameModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainFunc>()
            .register_typst_asset::<GameMode>()
            .compile_typst_func::<GameMode, MainFunc>()
            .recompile_on_interaction::<MainFunc>(|func| &mut func.dummy_update)
            .animate_resource::<MainFunc, f64>()
            .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    push_to_main_window::<MainFunc>().run_if(
                        |q_controller: Query<&SequenceController, With<AnimationMarker>>| {
                            q_controller.single().curr_time() > f32::EPSILON
                        },
                    ),
                    (sandbox_btn, matchmacke_btns, cancel_btn)
                        .run_if(|func: Res<MainFunc>| func.closing == false),
                    update_func_closing,
                )
                    .run_if(in_state(Screen::LocalLobby)),
            )
            .observe(show_game_modes);
    }
}

fn show_game_modes(
    trigger: Trigger<MatchmakeEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
) {
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();

    q_player.single_mut().time_scale = 1.0;
}

fn setup_animation(mut commands: Commands) {
    // Set up animations for cancel and spaceship buttons.
    let sequence = commands.play_motion(
        Action::<_, MainFunc>::new_f32lerp(Entity::PLACEHOLDER, 0.0, 1.0, |func| &mut func.animate)
            .with_ease(ease::cubic::ease_in_out)
            .animate(1.0),
    );

    commands.spawn((
        SequencePlayerBundle::from_sequence(sequence),
        AnimationMarker,
    ));
}

fn sandbox_btn(
    mut commands: Commands,
    interactions: InteractionQuery,
    mut q_seq_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
    mut evw_transparency: EventWriter<MainWindowTransparency>,
) {
    if interactions.pressed(SANDBOX_BTN) {
        // Hide menu.
        q_seq_player.single_mut().time_scale = -1.0;

        // Transition to sandbox screen.
        commands.add(Coroutine::new(move || {
            let mut res = co_break();
            res.add_subroutines((
                wait(std::time::Duration::from_secs_f32(WINDOW_FADE_DURATION)),
                move |mut connection_manager: ResMut<ConnectionManager>,
                      mut next_screen_state: ResMut<NextState<Screen>>| {
                    next_screen_state.set(Screen::Sandbox);
                    let _ = connection_manager.send_message::<OrdReliableChannel, _>(&EnterSandbox);

                    co_break()
                },
            ));
            res
        }));

        evw_transparency.send(MainWindowTransparency(0.0));
    }
}

fn matchmacke_btns(
    mut commands: Commands,
    interactions: InteractionQuery,
    mut q_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
    mut evw_transparency: EventWriter<MainWindowTransparency>,
) {
    let mut player_count = None;
    for (i, &btn) in MATCHMAKE_BTNS.iter().enumerate() {
        if interactions.pressed(btn) {
            player_count = Some(1 << (i + 1));
            break;
        }
    }

    let Some(player_count) = player_count else {
        return;
    };

    // Hide menu.
    q_player.single_mut().time_scale = -1.0;

    // Transition to matchmakinig screen.
    commands.add(Coroutine::new(move || {
        let mut res = co_break();
        res.add_subroutines((
            wait(std::time::Duration::from_secs_f32(WINDOW_FADE_DURATION)),
            move |mut connection_manager: ResMut<ConnectionManager>,
                  mut next_screen_state: ResMut<NextState<Screen>>,
                  mut lobby_func: ResMut<LobbyFunc>| {
                next_screen_state.set(Screen::Matchmaking);

                let _ = connection_manager
                    .send_message::<OrdReliableChannel, _>(&Matchmake(player_count));
                lobby_func.max_player_count = player_count;

                co_break()
            },
        ));
        res
    }));

    evw_transparency.send(MainWindowTransparency(0.0));
}

fn cancel_btn(
    interactions: InteractionQuery,
    mut q_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
) {
    if interactions.pressed(CANCEL_BTN) {
        q_player.single_mut().time_scale = -1.0;
    }
}

/// Update [`MainFunc::closing`] based on [`SequencePlayer::time_scale`].
fn update_func_closing(
    q_player: Query<&SequencePlayer, (Changed<SequencePlayer>, With<AnimationMarker>)>,
    mut func: ResMut<MainFunc>,
) {
    if let Ok(player) = q_player.get_single() {
        func.closing = player.time_scale < 0.0;
    }
}

#[derive(Component)]
struct AnimationMarker;

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    data: foundations::Dict,
    /// Animate time for showing/hiding the selection panel.
    animate: f64,
    /// When this is true, all button labels will be removed.
    closing: bool,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_mode.typ"]
struct GameMode;
