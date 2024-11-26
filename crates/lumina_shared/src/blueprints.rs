use blenvy::*;
use lumina_common::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter};

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/lobbies/")]
pub enum LobbyType {
    Local,
    Multiplayer,
}

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/spaceships/")]
pub enum SpaceshipType {
    Assassin,
    // Tank,
    // Support,
}

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/weapons/")]
pub enum WeaponType {
    Cannon,
    // Missle,
    // GattlingGun,
}

#[derive(
    Component,
    Reflect,
    EnumCount,
    EnumIter,
    AsRefStr,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[reflect(Component)]
#[strum(prefix = "levels/ammos/")]
pub enum AmmoType {
    LongRange,
    // ShortRange,
    // Honing,
}
enum_as_usize!(AmmoType);
