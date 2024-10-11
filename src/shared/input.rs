use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::INPUT_REPLICATION_GROUP;

use super::{player::PlayerId, LocalEntity};

#[derive(Bundle)]
pub struct ReplicateInputBundle {
    pub id: PlayerId,
    pub replicate: client::Replicate,
    pub input: InputManagerBundle<PlayerAction>,
    pub prepredicted: PrePredicted,
}

impl ReplicateInputBundle {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            replicate: client::Replicate {
                group: INPUT_REPLICATION_GROUP,
                ..default()
            },
            input: InputManagerBundle::with_map(PlayerAction::input_map()),
            prepredicted: PrePredicted::default(),
        }
    }
}

#[derive(Bundle)]
pub struct LocalInputBundle {
    pub input: InputManagerBundle<PlayerAction>,
    pub local: LocalEntity,
}

impl Default for LocalInputBundle {
    fn default() -> Self {
        Self {
            input: InputManagerBundle::with_map(PlayerAction::input_map()),
            local: LocalEntity,
        }
    }
}

#[derive(Actionlike, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Brake,
    Interact,
    UseItem,
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub(crate) fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Brake, GamepadButtonType::East);
        input_map.insert(Self::Interact, GamepadButtonType::South);
        input_map.insert(Self::UseItem, GamepadButtonType::RightTrigger2);

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Brake, KeyCode::Space);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MovementSet {
    // Input handling.
    Input,
    // Apply physics.
    Physics,
}
