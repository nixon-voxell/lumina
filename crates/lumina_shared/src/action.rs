use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::INPUT_REPLICATION_GROUP;

use super::player::PlayerId;

#[derive(Bundle)]
pub struct ReplicateActionBundle {
    pub id: PlayerId,
    pub input: InputManagerBundle<PlayerAction>,
    pub replicate: client::Replicate,
    pub prepredicted: PrePredicted,
}

impl ReplicateActionBundle {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            input: InputManagerBundle::with_map(PlayerAction::input_map()),
            replicate: client::Replicate {
                group: INPUT_REPLICATION_GROUP,
                ..default()
            },
            prepredicted: PrePredicted::default(),
        }
    }
}

#[derive(Bundle)]
pub struct LocalActionBundle {
    pub input: InputManagerBundle<PlayerAction>,
    pub id: PlayerId,
}

impl Default for LocalActionBundle {
    fn default() -> Self {
        Self {
            input: InputManagerBundle::with_map(PlayerAction::input_map()),
            id: PlayerId::LOCAL,
        }
    }
}

#[derive(Actionlike, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Brake,
    Boost,
    Interact,
    Attack,
    Aim,
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub(crate) fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Brake, GamepadButtonType::LeftTrigger);
        input_map.insert(Self::Boost, GamepadButtonType::LeftTrigger2);
        input_map.insert(Self::Interact, GamepadButtonType::South);
        input_map.insert(Self::Attack, GamepadButtonType::RightTrigger2);
        input_map.insert(Self::Aim, DualAxis::right_stick());

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Brake, KeyCode::Space);
        input_map.insert(Self::Boost, MouseButton::Right);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::Attack, MouseButton::Left);

        input_map
    }
}
