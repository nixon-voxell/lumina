use blenvy::*;
use lumina_common::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter};

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, blueprint_spawner::<TesseractType>);
    }
}

fn blueprint_spawner<T: BlueprintType>(
    mut commands: Commands,
    q_spawners: Query<(&BlueprintSpawner<T>, Entity), Added<BlueprintSpawner<T>>>,
) {
    for (spawner, entity) in q_spawners.iter() {
        let info = match spawner.spawn_type {
            SpawnType::Raw => spawner.blueprint.info(),
            SpawnType::Config => spawner.blueprint.config_info(),
            SpawnType::Visual => spawner.blueprint.visual_info(),
        };

        commands
            .entity(entity)
            .insert((info, SpawnBlueprint))
            .remove::<BlueprintSpawner<T>>();
    }
}

/// Spawn blueprint of given type on add.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlueprintSpawner<T: BlueprintType> {
    blueprint: T,
    spawn_type: SpawnType,
}

#[derive(Reflect)]
pub enum SpawnType {
    Raw,
    Config,
    Visual,
}

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
    MineSpot,
    // Exploding,
    // Timed,
    // Golden,
}
