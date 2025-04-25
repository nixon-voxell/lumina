use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::protocol::INPUT_REPLICATION_GROUP;

#[derive(Bundle)]
pub struct ReplicateActionBundle {
    pub id: PlayerId,
    pub input: InputManagerBundle<PlayerAction>,
    pub replicate: client::Replicate,
    pub prepredicted: PrePredicted,
    pub source: SourceEntity,
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
            source: SourceEntity,
        }
    }
}

#[derive(Actionlike, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Dash,
    Boost,
    Brake,
    Interact,
    Attack,
    Aim,
    Ability,
    Reload,
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Dash, GamepadButtonType::South);
        input_map.insert(Self::Boost, GamepadButtonType::LeftTrigger2);
        input_map.insert(Self::Brake, GamepadButtonType::North);
        input_map.insert(Self::Interact, GamepadButtonType::West);
        input_map.insert(Self::Attack, GamepadButtonType::RightTrigger2);
        input_map.insert(Self::Aim, DualAxis::right_stick());
        input_map.insert(Self::Ability, GamepadButtonType::East);
        input_map.insert(Self::Reload, GamepadButtonType::North);

        // KbM input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Dash, KeyCode::Space);
        input_map.insert(Self::Boost, KeyCode::ShiftLeft);
        // Additional option.
        input_map.insert(Self::Boost, MouseButton::Right);
        input_map.insert(Self::Brake, KeyCode::ControlLeft);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::Attack, MouseButton::Left);
        input_map.insert(Self::Ability, KeyCode::KeyQ);
        input_map.insert(Self::Reload, KeyCode::KeyR);

        input_map
    }
}
