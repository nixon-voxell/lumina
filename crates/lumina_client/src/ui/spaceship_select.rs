use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;

use crate::client::ConnectionManager;
use crate::effector::{InteractedEffector, SpaceshipSelectEffector};
use crate::typ_animation::AnimateTypAppExt;

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
            .init_resource::<MainFunc>()
            .init_resource::<ClientSpaceshipSelection>()
            .register_typst_asset::<SpaceshipSelect>()
            .compile_typst_func::<SpaceshipSelect, MainFunc>()
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
                    (handle_spaceship_selection, cancel_btn)
                        .run_if(|func: Res<MainFunc>| func.closing == false),
                    update_func_closing,
                )
                    .run_if(in_state(Screen::LocalLobby)),
            )
            .observe(show_spaceship_select);
    }
}

fn show_spaceship_select(
    trigger: Trigger<SpaceshipSelectEffector>,
    mut commands: Commands,
    mut q_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
) {
    commands
        .entity(trigger.entity())
        .remove::<InteractedEffector>();
    q_player.single_mut().time_scale = 1.0;
}

fn handle_spaceship_selection(
    interactions: InteractionQuery,
    mut selected: ResMut<ClientSpaceshipSelection>,
    mut q_player: Query<&mut SequencePlayer, With<AnimationMarker>>,
    mut evw_select_spaceship: EventWriter<SelectSpaceship>,
    mut evw_transparency: EventWriter<MainWindowTransparency>,
    mut connection_manager: ResMut<ConnectionManager>,
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
    let sequence = commands.play_motion(
        Action::<_, MainFunc>::new_f32lerp(Entity::PLACEHOLDER, 0.0, 1.0, |func| &mut func.animate)
            .with_ease(ease::cubic::ease_in_out)
            .animate(0.5),
    );

    commands.spawn((
        SequencePlayerBundle::from_sequence(sequence),
        AnimationMarker,
    ));
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
#[typst_path = "typst/client/spaceship_select.typ"]
struct SpaceshipSelect;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ClientSpaceshipSelection(pub SpaceshipType);
