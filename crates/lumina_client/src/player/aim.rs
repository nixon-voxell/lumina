use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::buttonlike::ButtonState;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::spaceship::Spaceship;
use lumina_shared::prelude::*;

use crate::camera::GameCamera;

use super::LocalPlayerInfo;

pub(super) struct AimPlugin;

impl Plugin for AimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            mouse_motion.in_set(InputManagerSystem::ManualControl),
        );
    }
}

fn mouse_motion(
    mut q_actions: Query<&mut ActionState<PlayerAction>, With<SourceEntity>>,
    q_spaceship_transforms: Query<Ref<Transform>, (With<Spaceship>, With<SourceEntity>)>,
    q_camera: Query<(&Camera, Ref<GlobalTransform>), With<GameCamera>>,
    mut cursor_evr: EventReader<CursorMoved>,
    local_player_info: LocalPlayerInfo,
    mut cursor_position: Local<Vec2>,
    mut is_using_mouse: Local<bool>,
) {
    let Some(mut action) = local_player_info
        .get(PlayerInfoType::Action)
        .and_then(|e| q_actions.get_mut(e).ok())
    else {
        return;
    };

    // Get spaceship transform
    let Some(spaceship_transform) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_spaceship_transforms.get(e).ok())
    else {
        return;
    };

    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    // Actions from the controller if Aim action is used before this update.
    if action.pressed(&PlayerAction::Aim) {
        *is_using_mouse = false;
    }

    // When mouse is moved, we use mouse instead.
    for cursor in cursor_evr.read() {
        *cursor_position = cursor.position;
        *is_using_mouse = true;
    }

    if *is_using_mouse {
        let cursor_world_position = camera
            .viewport_to_world_2d(&camera_transform, *cursor_position)
            .unwrap_or_default();

        let direction =
            (cursor_world_position - spaceship_transform.translation.xy()).normalize_or_zero();

        let action_data = action.action_data_mut_or_default(&PlayerAction::Aim);
        action_data.state = ButtonState::Pressed;
        action_data.axis_pair = Some(DualAxisData::from_xy(direction));
    }
}
