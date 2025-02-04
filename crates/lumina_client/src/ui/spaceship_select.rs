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

const SPACESHIP_BTNS: &[&str] = &["btn:defender", "btn:assassin"];
const CANCEL_BTN: &str = "btn:cancel-spaceship";

pub(super) struct SpaceshipSelectUiPlugin;

impl Plugin for SpaceshipSelectUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SpaceshipSelect>()
            .compile_typst_func::<SpaceshipSelect, SpaceshipFunc>()
            .init_resource::<SpaceshipFunc>()
            .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    push_to_main_window::<SpaceshipFunc>().run_if(
                        |q_controller: Query<&SequenceController, With<SpaceshipMainAnimation>>| {
                            q_controller.single().curr_time() > f32::EPSILON
                        },
                    ),
                    interactable_func::<SpaceshipFunc>,
                    spaceship_btns,
                    cancel_btn,
                )
                    .run_if(in_state(Screen::LocalLobby)), // Remain in lobby.
            )
            .observe(show_spaceship_select);
    }
}

fn show_spaceship_select(
    trigger: Trigger<SpaceshipSelectEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipMainAnimation>>,
) {
    // Remove the effector once it triggers.
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();
    q_player.single_mut().time_scale = 1.0;
}

fn setup_animation(mut commands: Commands) {
    // Set up animations for cancel and spaceship buttons.
    let sequences = [CANCEL_BTN]
        .iter()
        .chain(SPACESHIP_BTNS)
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
        SpaceshipMainAnimation,
    ));
}

fn spaceship_btns(
    mut commands: Commands,
    interactions: InteractionQuery,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipMainAnimation>>,
    mut transparency_evw: EventWriter<MainWindowTransparency>,
) {
    let mut selected_ship = None;
    for &btn in SPACESHIP_BTNS {
        if interactions.pressed(btn) {
            selected_ship = Some(btn);
            break;
        }
    }

    let Some(selected_ship) = selected_ship else {
        return;
    };

    // Reverse animation to hide the selection UI.
    q_player.single_mut().time_scale = -1.0;

    // Log the selection. Later, you can integrate your messaging and resource updates here.
    info!("Spaceship selected: {}", selected_ship);
    // TODO: Integrate spaceship selection handling (e.g., update LobbyFunc, send a message) later.

    transparency_evw.send(MainWindowTransparency(0.0));
}

fn cancel_btn(
    interactions: InteractionQuery,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipMainAnimation>>,
) {
    if interactions.pressed(CANCEL_BTN) {
        q_player.single_mut().time_scale = -1.0;
    }
}

#[derive(Component)]
pub struct SpaceshipMainAnimation;

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "spaceship_main", layer = 1)]
struct SpaceshipFunc {
    data: foundations::Dict,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
}

impl InteractableFunc for SpaceshipFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/spaceship_select.typ"]
struct SpaceshipSelect;
