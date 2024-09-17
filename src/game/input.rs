use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<ActionState<PlayerAction>>()
            .insert_resource(PlayerAction::input_map())
            .add_systems(
                Update,
                handle_player_input.run_if(in_state(GameState::InGame)),
            );
    }
}

// Handle player input actions
fn handle_player_input(action_state: Res<ActionState<PlayerAction>>) {
    if action_state.pressed(&PlayerAction::Move) {
        if let Some(axis_pair) = action_state.clamped_axis_pair(&PlayerAction::Move) {
            println!("Move: ({}, {})", axis_pair.x(), axis_pair.y());
        }
    }

    if action_state.pressed(&PlayerAction::Interact) {
        println!("jump");
    }

    if action_state.pressed(&PlayerAction::UseItem) {
        println!("use item");
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Interact,
    UseItem,
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub(crate) fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Interact, GamepadButtonType::South);
        input_map.insert(Self::UseItem, GamepadButtonType::RightTrigger2);

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Interact, KeyCode::Space);
        input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}
