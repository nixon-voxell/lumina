use bevy::prelude::*;

use crate::effector::*;

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Effector
            .register_type::<EffectorPopupMsg>()
            .register_type::<InteractableEffector>()
            .register_type::<Effector>()
            .register_type::<MatchmakeEffector>()
            .register_type::<TesseractEffector>()
            .register_type::<SpaceshipSelectEffector>()
            .register_type::<TeleporterEffector>();
    }
}
