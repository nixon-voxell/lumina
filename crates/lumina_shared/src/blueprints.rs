use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter};

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                blueprint_spawner::<TesseractType>,
                blueprint_spawner::<OreType>,
                despawn_on_spawn,
            ),
        );
    }
}

fn blueprint_spawner<T: BlueprintType>(
    mut commands: Commands,
    q_spawners: Query<(&BlueprintSpawner<T>, Entity), Added<BlueprintSpawner<T>>>,
    network_identity: NetworkIdentity,
) {
    for (spawner, entity) in q_spawners.iter() {
        // Despawn for target that doesn't match.
        if spawner.target.is_some_and(|target| {
            matches!(
                (network_identity.is_server(), target),
                (true, NetworkIdentityTarget::Client) | (false, NetworkIdentityTarget::Server)
            )
        }) {
            commands.entity(entity).despawn();
            continue;
        }

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

fn despawn_on_spawn(
    mut commands: Commands,
    q_despawners: Query<(&DespawnOnSpawn, Entity), Added<DespawnOnSpawn>>,
    network_identity: NetworkIdentity,
) {
    for (despawner, entity) in q_despawners.iter() {
        // Ignore if target doesn't match.
        if matches!(
            (network_identity.is_server(), despawner.target),
            (true, NetworkIdentityTarget::Client) | (false, NetworkIdentityTarget::Server)
        ) {
            continue;
        }

        if despawner.recursive {
            commands.entity(entity).despawn_recursive();
        } else {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn blueprint of given type on add.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlueprintSpawner<T: BlueprintType> {
    blueprint: T,
    spawn_type: SpawnType,
    /// Specifies a [NetworkIdentityTarget] if [Some] both if [None].
    target: Option<NetworkIdentityTarget>,
}

/// Despawn entity for either client or server.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct DespawnOnSpawn {
    target: NetworkIdentityTarget,
    recursive: bool,
}

impl Default for DespawnOnSpawn {
    fn default() -> Self {
        Self {
            target: NetworkIdentityTarget::Client,
            recursive: true,
        }
    }
}

#[derive(Reflect, Clone, Copy)]
pub enum NetworkIdentityTarget {
    Client,
    Server,
}

#[derive(Reflect, Clone, Copy)]
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
    Defender,
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
