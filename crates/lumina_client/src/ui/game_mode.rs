use bevy::prelude::*;
use bevy_coroutine::prelude::*;
use bevy_motiongfx::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;
use velyst::typst_element::prelude::*;

use crate::effector::InteractedEffector;
use crate::typ_animation::LabelScaleFade;

use super::lobby::LobbyFunc;
use super::Screen;

const MATCHMAKE_BTNS: &[&str] = &["btn:1v1", "btn:2v2", "btn:3v3"];
const SANDBOX_BTN: &str = "btn:sandbox";

pub(super) struct GameModeUiPlugin;

impl Plugin for GameModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameMode>()
            .compile_typst_func::<GameMode, MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    push_to_main_window::<MainFunc>().run_if(
                        |q_controller: Query<&SequenceController, With<MainFuncAnimation>>| {
                            q_controller.single().curr_time() > f32::EPSILON
                        },
                    ),
                    interactable_func::<MainFunc>,
                    (sandbox_btn, matchmacke_btns, cancel_btn).run_if(is_panel_open),
                )
                    .run_if(in_state(Screen::LocalLobby)),
            )
            .observe(show_game_modes);
    }
}

fn show_game_modes(
    trigger: Trigger<MatchmakeEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<MainFuncAnimation>>,
) {
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();

    q_player.single_mut().time_scale = 1.0;
}

fn setup_animation(mut commands: Commands) {
    let sequences = ["btn:cancel-matchmake", SANDBOX_BTN]
        .iter()
        .chain(MATCHMAKE_BTNS)
        .map(|&btn| {
            let id = commands.spawn(LabelScaleFade::new(btn)).id();
            commands.play_motion(
                Action::<_, LabelScaleFade>::new_f32lerp(id, 0.0, 1.0, |label| &mut label.time)
                    .with_ease(ease::cubic::ease_in_out)
                    .animate(0.4),
            )
        })
        .collect::<Vec<_>>();

    commands.spawn((
        SequencePlayerBundle::from_sequence(sequences.flow(0.1)),
        MainFuncAnimation,
    ));
}

fn sandbox_btn(
    mut commands: Commands,
    interactions: InteractionQuery,
    mut q_seq_player: Query<&mut SequencePlayer, With<MainFuncAnimation>>,
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
    mut q_seq_player: Query<&mut SequencePlayer, With<MainFuncAnimation>>,
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
    q_seq_player.single_mut().time_scale = -1.0;

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
    mut q_player: Query<&mut SequencePlayer, With<MainFuncAnimation>>,
) {
    if interactions.pressed("btn:cancel-matchmake") {
        q_player.single_mut().time_scale = -1.0;
    }
}

fn is_panel_open(q_seq_player: Query<&SequencePlayer, With<MainFuncAnimation>>) -> bool {
    q_seq_player.single().time_scale > 0.0
}

#[derive(Component)]
pub struct MainFuncAnimation;

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    data: foundations::Dict,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
}

impl InteractableFunc for MainFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_mode.typ"]
struct GameMode;
