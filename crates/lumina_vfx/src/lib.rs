use bevy::prelude::*;
use bevy::render::render_resource::*;

mod material2d;
mod particle;
mod type_registry;

pub mod prelude {
    pub use crate::material2d::BoosterMaterial;
    pub use crate::particle::MuzzleFlashMaterial;
    pub use crate::particle::{DespawnVfx, DespawnVfxEffects, DespawnVfxType};
    pub use crate::particle::{
        InPlaceVfxAssets, InPlaceVfxMap, InPlaceVfxMapPlugin, InPlaceVfxType,
    };
}

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            material2d::Material2dVfxPlugin,
            particle::ParticleVfxPlugin,
            type_registry::TypeRegistryPlugin,
        ));
    }
}

pub const BLEND_ADD: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },
    alpha: BlendComponent {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },
};
