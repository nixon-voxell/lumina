use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy_enoki::prelude::*;

mod material2d;
mod particle;
mod type_registry;

pub mod prelude {
    pub use crate::material2d::BoosterMaterial;
    pub use crate::particle::{AmmoHitMaterial, MuzzleFlashMaterial};
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
        ))
        .add_systems(Update, init_oneshot_effect);
    }
}

fn init_oneshot_effect(mut commands: Commands, q_one_shots: Query<Entity, Added<OneShotEffect>>) {
    for entity in q_one_shots.iter() {
        commands.entity(entity).insert(OneShot::Deactivate);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct OneShotEffect;

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