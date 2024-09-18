use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::client::*;

use crate::game::{input::PlayerAction, player::PlayerTransform};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (movement, apply_transform));
    }
}

/// Player movement
fn movement(
    mut q_player_transform: Query<&mut PlayerTransform, With<Predicted>>,
    action_state: Res<ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 100.0;

    let Ok(mut player_transform) = q_player_transform.get_single_mut() else {
        return;
    };

    if action_state.pressed(&PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
            player_transform.translation +=
                axis_pair.xy().normalize() * time.delta_seconds() * SPEED;
        }
    }
}

/// Apply [`Transform`] from [`PlayerTransform`].
fn apply_transform(
    mut q_transforms: Query<(&mut Transform, &PlayerTransform), Changed<PlayerTransform>>,
) {
    for (mut transform, player_transform) in q_transforms.iter_mut() {
        transform.translation.x = player_transform.translation.x;
        transform.translation.y = player_transform.translation.y;
        transform.rotation = Quat::from_rotation_z(player_transform.rotation);
    }
}

/// System that reads from peripherals and adds inputs to the buffer
/// This system must be run in the `InputSystemSet::BufferInputs` set in the `FixedPreUpdate` schedule
/// to work correctly.
///
/// I would also advise to use the `leafwing` feature to use the `LeafwingInputPlugin` instead of the
/// `InputPlugin`, which contains more features.
pub(crate) fn buffer_input(
    tick_manager: Res<TickManager>,
    mut input_manager: ResMut<InputManager<Inputs>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    let tick = tick_manager.tick();
    let mut input = Inputs::None;
    let mut direction = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
        direction.up = true;
    }
    if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
        direction.down = true;
    }
    if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
        direction.left = true;
    }
    if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
        direction.right = true;
    }
    if !direction.is_none() {
        input = Inputs::Direction(direction);
    }
    if keypress.pressed(KeyCode::Backspace) {
        input = Inputs::Delete;
    }
    if keypress.pressed(KeyCode::Space) {
        input = Inputs::Spawn;
    }
    input_manager.add_input(input, tick)
}
