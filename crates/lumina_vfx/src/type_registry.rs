use bevy::prelude::*;

use super::prelude::*;

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BoosterMaterial>();
    }
}
