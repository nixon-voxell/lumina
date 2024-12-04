use bevy::prelude::*;
use bevy_radiance_cascades::prelude::*;
use lumina_common::physics::{MeshRigidbody, PrimitiveRigidbody};

use crate::blueprints::{AmmoType, SpaceshipType, WeaponType};
use crate::effector::{
    Effector, EffectorPopupMsg, InteractableEffector, MatchmakeEffector, TutorialEffector,
};
use crate::health::{Health, MaxHealth};
use crate::player::ammo::AmmoRef;
use crate::player::prelude::*;
use crate::player::spaceship::Boost;

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Radiance
            .register_type::<NoRadiance>()
            // Health
            .register_type::<MaxHealth>()
            .register_type::<Health>()
            // Game
            .register_type::<SpawnPoint>()
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
            .register_type::<TutorialEffector>()
            // Physics
            .register_type::<PrimitiveRigidbody>()
            .register_type::<MeshRigidbody>();
    }
}
