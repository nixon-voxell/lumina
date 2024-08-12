use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    Run,
    Jump,
    UseItem,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Run => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub(crate) fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(Self::Run, GamepadStick::LEFT);
        input_map.insert(Self::Jump, GamepadButtonType::South);
        input_map.insert(Self::UseItem, GamepadButtonType::RightTrigger2);

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Run, KeyboardVirtualDPad::WASD);
        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}
