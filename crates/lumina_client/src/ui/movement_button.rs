use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict, IntoValue};

use crate::camera::GameCamera;
use crate::player::aim::IsUsingMouse;
use crate::player::LocalPlayerInfo;
use crate::screens::Screen;

pub(super) struct MovementButtonUiPlugin;

impl Plugin for MovementButtonUiPlugin {
    fn build(&self, app: &mut App) {
        let run_condition = in_state(Screen::LocalLobby);

        app.register_typst_asset::<MovementButton>()
            .compile_typst_func::<MovementButton, MainFunc>()
            .push_to_main_window::<MovementButton, MainFunc, _>(
                MainWindowSet::Default,
                run_condition.clone(),
            )
            .init_resource::<MainFunc>()
            .add_systems(
                PostUpdate,
                movement_button
                    .in_set(MainWindowTransformSyncSet)
                    .run_if(run_condition),
            );
    }
}

fn movement_button(
    q_actions: Query<&ActionState<PlayerAction>, With<SourceEntity>>,
    q_game_camera: Query<(&GlobalTransform, &OrthographicProjection, &Camera), With<GameCamera>>,
    mut func: ResMut<MainFunc>,
    is_using_mouse: Res<IsUsingMouse>,
    local_player_info: LocalPlayerInfo,
) {
    let Some(action) = local_player_info
        .get(PlayerInfoType::Action)
        .and_then(|e| q_actions.get(e).ok())
    else {
        return;
    };

    let Ok((camera_transform, proj, camera)) = q_game_camera.get_single() else {
        return;
    };
    // Origin relative to the camera.
    let origin = camera
        .world_to_viewport(camera_transform, Vec3::ZERO)
        .unwrap_or_default()
        .as_dvec2();

    let mut act = dict! {
        "dash" => action.pressed(&PlayerAction::Dash),
        "boost" => action.pressed(&PlayerAction::Boost),
        "interact" => action.pressed(&PlayerAction::Interact),
        "attack" => action.pressed(&PlayerAction::Attack),
        "ability" => action.pressed(&PlayerAction::Ability),
        "reload" => action.pressed(&PlayerAction::Reload),
    };

    let move_action = action
        .clamped_axis_pair(&PlayerAction::Move)
        .unwrap_or_default();

    if is_using_mouse.0 {
        act.insert("w".into(), (move_action.y() == 1.0).into_value());
        act.insert("a".into(), (move_action.x() == -1.0).into_value());
        act.insert("s".into(), (move_action.y() == -1.0).into_value());
        act.insert("d".into(), (move_action.x() == 1.0).into_value());
    } else {
        act.insert("move_x".into(), (move_action.x() as f64).into_value());
        act.insert("move_y".into(), (move_action.y() as f64).into_value());

        let aim_action = action
            .clamped_axis_pair(&PlayerAction::Aim)
            .unwrap_or_default();

        act.insert("aim_x".into(), (aim_action.x() as f64).into_value());
        act.insert("aim_y".into(), (aim_action.y() as f64).into_value());
    }

    func.data = dict! {
        "scale" => proj.scale as f64,
        "origin_x" => origin.x,
        "origin_y" => origin.y,
        "is_using_mouse" => is_using_mouse.0,
        "act" => act,
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 0)]
struct MainFunc {
    data: Dict,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/movement_button.typ"]
pub struct MovementButton;
