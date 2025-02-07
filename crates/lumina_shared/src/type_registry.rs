use bevy::prelude::*;
use bevy_radiance_cascades::prelude::*;

use crate::blueprints::*;
use crate::effector::*;
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
            .register_type::<BlueprintSpawner<TesseractType>>()
            .register_type::<BlueprintSpawner<OreType>>()
            .register_type::<DespawnOnSpawn>()
            // Game
            .register_type::<MaxHealth>()
            .register_type::<Health>()
            .register_type::<SpawnPoint>()
            .register_type::<TesseractType>()
            .register_type::<OreType>()
            .register_type::<LuminaType>()
            .register_type::<LuminaStat>()
            .register_type::<ObjectiveArea>()
            .register_type::<LuminaSpawnArea>()
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
            // Effector
            .register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>()
            .register_type::<Effector>()
            .register_type::<MatchmakeEffector>()
            .register_type::<TesseractEffector>()
            .register_type::<SpaceshipSelectEffector>();
    }
}
