use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::buttonlike::ButtonState;
use leafwing_input_manager::prelude::*;

use crate::client::camera::GameCamera;
use crate::client::PrePredictedOrLocal;
use crate::shared::action::PlayerAction;
use crate::shared::player::LocalPlayer;

pub(super) struct AimPlugin;

impl Plugin for AimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_motion);
    }
}

fn mouse_motion(
    mut q_action: Query<&mut ActionState<PlayerAction>, PrePredictedOrLocal>,
    q_player: Query<&Transform, With<LocalPlayer>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut cursor_evr: EventReader<CursorMoved>,
) {
    let Ok(mut action) = q_action.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = q_player.get_single() else {
        return;
    };

    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    for cursor in cursor_evr.read() {
        let cursor_world_position = camera
            .viewport_to_world_2d(camera_transform, cursor.position)
            .unwrap_or_default();

        let direction =
            (cursor_world_position - player_transform.translation.xy()).normalize_or_zero();

        let action_data = action.action_data_mut_or_default(&PlayerAction::Aim);
        action_data.state = ButtonState::Pressed;
        action_data.axis_pair = Some(DualAxisData::from_xy(direction));
    }
}
