use bevy::prelude::*;
use lumina_common::physics::{MeshRigidbody, PrimitiveRigidbody};

use crate::effector::{
    Effector, EffectorPopupMsg, InteractableEffector, MatchmakeEffector, TutorialEffector,
};
use crate::player::ammo::{Ammo, AmmoRef, AmmoType};
use crate::player::spaceship::{Spaceship, SpaceshipType};
use crate::player::weapon::{Weapon, WeaponType};

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        // Player
        app.register_type::<Weapon>()
            .register_type::<WeaponType>()
            .register_type::<Spaceship>()
            .register_type::<SpaceshipType>()
            .register_type::<Ammo>()
            .register_type::<AmmoType>()
            .register_type::<AmmoRef>();

        // Effector
        app.register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>()
            .register_type::<Effector>()
            .register_type::<MatchmakeEffector>()
            .register_type::<TutorialEffector>();

        // Physics
        app.register_type::<PrimitiveRigidbody>()
            .register_type::<MeshRigidbody>();
    }
}
