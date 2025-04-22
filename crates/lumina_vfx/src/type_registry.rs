use bevy::prelude::*;

use crate::OneShotEffect;

use super::prelude::*;

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OneShotEffect>()
            .register_type::<PortalMaterial>()
            .register_type::<BoosterMaterial>()
            .register_type::<HealAbilityMaterial>()
            .register_type::<MuzzleFlashMaterial>()
            .register_type::<InPlaceVfxType>();
    }
}
