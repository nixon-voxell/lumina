use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;
use velyst::typst_element::prelude::*;

use crate::client::ConnectionManager;
use crate::effector::{InteractedEffector, SpaceshipSelectEffector};
use crate::typ_animation::LabelScaleFade;
use crate::LocalClientId;

use lumina_shared::protocol::SelectSpaceship;

use super::Screen;

const SPACESHIP_BTNS: &[(&str, SpaceshipType)] = &[
    ("btn:defender", SpaceshipType::Defender),
    ("btn:assassin", SpaceshipType::Assassin),
];
const CANCEL_BTN: &str = "btn:cancel-spaceship";

pub(super) struct SpaceshipSelectUiPlugin;

impl Plugin for SpaceshipSelectUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectSpaceship>()
            .register_typst_asset::<SpaceshipSelect>()
            .compile_typst_func::<SpaceshipSelect, SpaceshipSelectFunc>()
            .init_resource::<ClientSpaceshipSelection>()
            .init_resource::<SpaceshipSelectFunc>()
            .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    push_to_main_window::<SpaceshipSelectFunc>().run_if(
                        |q_controller: Query<
                            &SequenceController,
                            With<SpaceshipSelectFuncAnim>,
                        >| {
                            q_controller.single().curr_time() > f32::EPSILON
                        },
                    ),
                    interactable_func::<SpaceshipSelectFunc>,
                    (handle_spaceship_selection, cancel_btn).run_if(is_panel_open),
                )
                    .run_if(in_state(Screen::LocalLobby)),
            )
            .observe(show_spaceship_select);
    }
}

fn show_spaceship_select(
    trigger: Trigger<SpaceshipSelectEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipSelectFuncAnim>>,
) {
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();
    q_player.single_mut().time_scale = 1.0;
}

fn handle_spaceship_selection(
    interactions: InteractionQuery,
    mut selected: ResMut<ClientSpaceshipSelection>,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipSelectFuncAnim>>,
    mut evw_select_spaceship: EventWriter<SelectSpaceship>,
    mut evw_transparency: EventWriter<MainWindowTransparency>,
    mut connection_manager: ResMut<ConnectionManager>,
    _local_client_id: Res<LocalClientId>,
) {
    for &(btn, ship_type) in SPACESHIP_BTNS {
        if interactions.pressed(btn) {
            **selected = ship_type;
            q_player.single_mut().time_scale = -1.0;
            evw_transparency.send(MainWindowTransparency(1.0));

            // Construct the message using the new tuple struct.
            let select_msg = SelectSpaceship(ship_type);
            match connection_manager.send_message::<OrdReliableChannel, _>(&select_msg) {
                Err(e) => error!("Failed to send SelectSpaceship message: {:?}", e),
                Ok(_) => info!("Spaceship selected: {:?}", ship_type),
            }
            evw_select_spaceship.send(select_msg);
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
        SpaceshipSelectFuncAnim,
    ));
}

fn cancel_btn(
    interactions: InteractionQuery,
    mut q_player: Query<&mut SequencePlayer, With<SpaceshipSelectFuncAnim>>,
) {
    if interactions.pressed(CANCEL_BTN) {
        q_player.single_mut().time_scale = -1.0;
    }
}

fn is_panel_open(q_seq_player: Query<&SequencePlayer, With<SpaceshipSelectFuncAnim>>) -> bool {
    q_seq_player.single().time_scale > 0.0
}

#[derive(Component)]
pub struct SpaceshipSelectFuncAnim;

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "spaceship_main", layer = 1)]
struct SpaceshipSelectFunc {
    data: foundations::Dict,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
}

impl InteractableFunc for SpaceshipSelectFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/spaceship_select.typ"]
struct SpaceshipSelect;

#[derive(Resource, Deref, DerefMut)]
pub struct ClientSpaceshipSelection(pub SpaceshipType);

impl Default for ClientSpaceshipSelection {
    fn default() -> Self {
        Self(SpaceshipType::Assassin)
    }
}
