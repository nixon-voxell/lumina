use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use super::player::PlayerId;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LeafwingInputPlugin::<PlayerAction>::default());
    }
}

#[derive(Bundle)]
pub struct ReplicateInputBundle {
    pub id: PlayerId,
    pub replicate: client::Replicate,
    pub input: InputManagerBundle<PlayerAction>,
    pub prepredicted: PrePredicted,
}

#[derive(Actionlike, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
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
