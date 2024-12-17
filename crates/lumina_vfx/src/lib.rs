use bevy::prelude::*;

mod material2d;
mod type_registry;

pub mod prelude {
    pub use crate::material2d::BoosterMaterial;
}

pub struct VfxPlugin;

// TODO: Fix bevy_motiongfx

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            material2d::Material2dVfxPlugin,
            type_registry::TypeRegistryPlugin,
        ));
    }
}
