use blenvy::*;
use lightyear::prelude::*;
use strum::{AsRefStr, EnumCount, EnumIter};

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Marker for replicating the entity over the network.
/// A [`server::Replicate`] bundle will be inserted on the server.
///
/// Entity will be removed from the client recursively.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ReplicateFromServer;

/// Marker for replicating the entity inside the children hierarchy
/// of [`ReplicateFromServer`] over the network.
/// The entity will be added to the room accordingly on the server.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HierarchySync;

/// Marker for replicating the [`BlueprintInfo`] over the network.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ReplicateBlueprint {
    /// The path in [`BlueprintInfo`].
    #[reflect(ignore)]
    pub path: String,
}

impl ReplicateBlueprint {
    pub fn info(&self) -> BlueprintInfo {
        BlueprintInfo::from_path(&self.path)
    }
}

/// Marker for entities to be spawned on server only.
/// Use [`ReplicateFromServer`] if the entity needs to be replicated
/// back to the clients.
///
/// Entity will be removed from the client recursively.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ServerOnly;

/// Marker for entities to be spawned on client only.
///
/// Entity will be removed from the server recursively.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ClientOnly;

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/maps/")]
pub enum MapType {
    Local,
    Multiplayer,
    Sandbox,
    AbandonedFactory,
}

#[derive(
    Component, Reflect, AsRefStr, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq,
)]
#[reflect(Component)]
#[strum(prefix = "levels/spaceships/")]
pub enum SpaceshipType {
    #[default]
    Assassin,
    Defender,
}

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/weapons/")]
pub enum WeaponType {
    Cannon,
    GattlingGun,
    // Missle,
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
#[strum(prefix = "levels/ores/")]
pub enum OreType {
    /// Drops 1-2 [LuminaType::Normal].
    Small,
    /// Drops 3-5 [LuminaType::Normal].
    Medium,
    /// Drops 5-8 [LuminaType::Normal].
    Large,
}

impl OreType {
    /// Calculate random value based on ore type.
    pub fn rand_value(&self) -> u8 {
        let mut rand_val = rand::random();
        rand_val = match self {
            OreType::Small => (rand_val % 2) + 1,
            OreType::Medium => (rand_val % 3) + 3,
            OreType::Large => (rand_val % 4) + 5,
        };

        rand_val
    }
}

#[derive(Component, Reflect, AsRefStr, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
#[strum(prefix = "levels/luminas/")]
pub enum LuminaType {
    Normal,
}
