use bevy::prelude::*;
use bevy_radiance_cascades::prelude::*;
use lumina_common::physics::{MeshRigidbody, PrimitiveRigidbody};

use crate::blueprints::{AmmoType, SpaceshipType, TesseractType, WeaponType};
use crate::effector::{
    Effector, EffectorPopupMsg, InteractableEffector, MatchmakeEffector, TesseractEffector,
};
use crate::health::{Health, MaxHealth};
use crate::player::ammo::AmmoRef;
use crate::player::objective::LuminaStat;
use crate::player::prelude::*;
use crate::player::spaceship::Boost;
use crate::prelude::{BlueprintSpawner, LuminaType};

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlueprintSpawner<TesseractType>>()
            // Radiance
            .register_type::<NoRadiance>()
            // Health
            .register_type::<MaxHealth>()
            .register_type::<Health>()
            // Game
            .register_type::<SpawnPoint>()
            .register_type::<TesseractType>()
            .register_type::<LuminaType>()
            .register_type::<LuminaStat>()
            // Player
            .register_type::<Weapon>()
            .register_type::<WeaponType>()
            .register_type::<Spaceship>()
            .register_type::<SpaceshipType>()
            .register_type::<Boost>()
            .register_type::<Ammo>()
            .register_type::<AmmoType>()
            .register_type::<AmmoRef>()
            // Effector
            .register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>()
            .register_type::<Effector>()
            .register_type::<MatchmakeEffector>()
            .register_type::<TesseractEffector>()
            // Physics
            .register_type::<PrimitiveRigidbody>()
            .register_type::<MeshRigidbody>();
    }
}
