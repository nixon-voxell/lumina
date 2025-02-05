use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;
use velyst::typst_element::prelude::*;

use crate::client::ConnectionManager;
use crate::effector::InteractedEffector;
use crate::typ_animation::LabelScaleFade;
use crate::LocalClientId;

use lumina_shared::protocol::SelectSpaceship;

use super::Screen;

const SPACESHIP_BTNS: &[(&str, SpaceshipType)] = &[
    ("btn:defender", SpaceshipType::Defender),
    ("btn:assassin", SpaceshipType::Assassin),
];
const CANCEL_BTN: &str = "btn:cancel-spaceship";

#[derive(Resource, Default)]
pub struct SelectedSpaceship(pub Option<SpaceshipType>);

pub(super) struct SpaceshipSelectUiPlugin;

impl Plugin for SpaceshipSelectUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SpaceshipSelect>()
            .compile_typst_func::<SpaceshipSelect, SpaceshipFunc>()
            .init_resource::<SelectedSpaceship>()
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
                    handle_spaceship_selection,
                    cancel_btn,
                )
                    .run_if(in_state(Screen::LocalLobby)),
            )
            .observe(show_spaceship_select);
    }
}

fn show_spaceship_select(
    trigger: Trigger<SpaceshipSelectEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipMainAnimation>>,
) {
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();
    q_player.single_mut().time_scale = 1.0;
}

fn handle_spaceship_selection(
    interactions: InteractionQuery,
    mut selected: ResMut<SelectedSpaceship>,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipMainAnimation>>,
    mut transparency_evw: EventWriter<MainWindowTransparency>,
    mut connection_manager: ResMut<ConnectionManager>,
    _local_client_id: Res<LocalClientId>,
) {
    for &(btn, ship_type) in SPACESHIP_BTNS {
        if interactions.pressed(btn) {
            selected.0 = Some(ship_type);
            q_player.single_mut().time_scale = -1.0;
            transparency_evw.send(MainWindowTransparency(1.0));

            let join_msg = SelectSpaceship {
                spaceship: ship_type,
            };
            if let Err(e) = connection_manager.send_message::<OrdReliableChannel, _>(&join_msg) {
                error!("Failed to send JoinLobby message: {:?}", e);
            } else {
                info!("Spaceship selected: {:?}", ship_type);
            }
            break;
        }
    }
}

fn setup_animation(mut commands: Commands) {
    // Set up animations for cancel and spaceship buttons.
    let sequences = std::iter::once(CANCEL_BTN)
        .chain(SPACESHIP_BTNS.iter().map(|(btn, _)| *btn))
        .map(|btn| {
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
