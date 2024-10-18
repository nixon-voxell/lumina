use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::buttonlike::ButtonState;
use leafwing_input_manager::prelude::*;

use crate::client::camera::GameCamera;
use crate::shared::action::PlayerAction;
use crate::shared::player::spaceship::SpaceShip;
use crate::shared::player::PlayerInfoType;
use crate::shared::SourceEntity;

use super::LocalPlayerInfo;

pub(super) struct AimPlugin;

impl Plugin for AimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_motion);
    }
}

fn mouse_motion(
    mut q_action: Query<&mut ActionState<PlayerAction>, With<SourceEntity>>,
    q_spaceship_transforms: Query<&Transform, (With<SpaceShip>, With<SourceEntity>)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut cursor_evr: EventReader<CursorMoved>,
    local_player_info: LocalPlayerInfo,
) {
    let Ok(mut action) = q_action.get_single_mut() else {
        return;
    };

    let Some(spaceship_transform) = local_player_info
        .get(PlayerInfoType::SpaceShip)
        .and_then(|e| q_spaceship_transforms.get(e).ok())
    else {
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
            (cursor_world_position - spaceship_transform.translation.xy()).normalize_or_zero();

        let action_data = action.action_data_mut_or_default(&PlayerAction::Aim);
        action_data.state = ButtonState::Pressed;
        action_data.axis_pair = Some(DualAxisData::from_xy(direction));
    }
}
