use blenvy::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter};

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/lobbies/")]
pub enum LobbyType {
    Local,
    Multiplayer,
    Sandbox,
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

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/tesseracts/")]
pub enum TesseractType {
    Tesseract,
}

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/luminas/")]
pub enum LuminaType {
    Normal,
    // Exploding,
    // Timed,
    // Golden,
}
