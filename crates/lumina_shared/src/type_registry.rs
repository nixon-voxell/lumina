use bevy::prelude::*;
use bevy_radiance_cascades::prelude::*;

use crate::blueprints::*;
use crate::game::prelude::*;
use crate::health::{Health, MaxHealth};
use crate::player::ammo::AmmoRef;
use crate::player::objective::{LuminaSpawnArea, LuminaStat, ObjectiveArea};
use crate::player::prelude::*;
use crate::player::spaceship::{EnergyConfig, Spaceship};

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Radiance
            .register_type::<NoRadiance>()
            // Blueprint
            .register_type::<ReplicateFromServer>()
            .register_type::<HierarchySync>()
            .register_type::<ReplicateBlueprint>()
            .register_type::<ServerOnly>()
            .register_type::<ClientOnly>()
            // Game
            .register_type::<MaxHealth>()
            .register_type::<Health>()
            .register_type::<SpawnPointParent>()
            .register_type::<SpawnPoint>()
            .register_type::<TesseractType>()
            .register_type::<OreType>()
            .register_type::<LuminaType>()
            .register_type::<LuminaStat>()
            .register_type::<ObjectiveArea>()
            .register_type::<LuminaSpawnArea>()
            .register_type::<TeleporterStart>()
            .register_type::<TeleporterEnd>()
            .register_type::<Teleporter>()
            .register_type::<Animator>()
            .register_type::<Playback>()
            // Player
            .register_type::<Weapon>()
            .register_type::<WeaponType>()
            .register_type::<Spaceship>()
            .register_type::<Spaceship>()
            .register_type::<EnergyConfig>()
            .register_type::<SpaceshipType>()
            .register_type::<Ammo>()
            .register_type::<AmmoType>()
            .register_type::<AmmoRef>()
            .register_type::<ShadowAbilityConfig>()
            .register_type::<HealAbilityConfig>();
    }
}
