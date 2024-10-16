use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::protocol::INPUT_REPLICATION_GROUP;

use super::{player::PlayerId, LocalEntity};

#[derive(Bundle)]
pub struct ReplicateInputBundle {
    pub id: PlayerId,
    pub input: InputManagerBundle<PlayerAction>,
    pub replicate: client::Replicate,
    pub prepredicted: PrePredicted,
}

impl ReplicateInputBundle {
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
pub struct LocalInputBundle {
    pub input: InputManagerBundle<PlayerAction>,
    pub local: LocalEntity,
    pub target: InputTarget,
}

impl LocalInputBundle {
    pub fn new(target: InputTarget) -> Self {
        Self {
            input: InputManagerBundle::with_map(PlayerAction::input_map()),
            local: LocalEntity,
            target,
        }
    }
}

/// The entity that the input is targetting.
#[derive(Component, Deref)]
pub struct InputTarget(Entity);

impl InputTarget {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

#[derive(Actionlike, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Brake,
    Boost,
    Interact,
    UseItem,
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
        input_map.insert(Self::UseItem, GamepadButtonType::RightTrigger2);
        input_map.insert(Self::Aim, DualAxis::right_stick());

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Brake, KeyCode::Space);
        input_map.insert(Self::Boost, MouseButton::Right);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}
