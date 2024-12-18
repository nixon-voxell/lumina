use std::hash::Hash;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dKey, Material2dPlugin};

pub(super) struct Material2dVfxPlugin;

impl Plugin for Material2dVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dFromComponentPlugin::<BoosterMaterial>::default());
    }
}

#[derive(Component, Reflect, Asset, AsBindGroup, Debug, Clone)]
#[reflect(Component)]
pub struct BoosterMaterial {
    #[uniform(0)]
    pub primary_color: LinearRgba,
    #[uniform(1)]
    pub secondary_color: LinearRgba,
    #[uniform(2)]
    pub rotation: f32,
    #[uniform(3)]
    pub inv_scale: f32,
    #[uniform(4)]
    pub ignition: f32,
}

impl Material2d for BoosterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vfx/booster.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target_state) = &mut fragment.targets[0] {
                target_state.blend = Some(BLEND_ADD);
            }
        }

        Ok(())
    }
}

const BLEND_ADD: BlendState = BlendState {
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

pub struct Material2dFromComponentPlugin<T: Component + Material2d + Clone>(PhantomData<T>);

impl<T: Component + Material2d + Clone> Plugin for Material2dFromComponentPlugin<T>
where
    T::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<T>::default())
            .add_systems(
                PostUpdate,
                (
                    replicate_asset_from_component::<T>,
                    update_asset_from_component::<T>,
                ),
            );
    }
}

impl<T: Component + Material2d + Clone> Default for Material2dFromComponentPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

fn replicate_asset_from_component<T: Component + Asset + Clone>(
    mut commands: Commands,
    q_components: Query<(&T, Entity), Added<T>>,
    mut assets: ResMut<Assets<T>>,
) {
    for (comp, entity) in q_components.iter() {
        commands
            .entity(entity)
            .insert(assets.add(comp.clone()))
            .remove::<Handle<ColorMaterial>>();
    }
}

fn update_asset_from_component<T: Component + Asset + Clone>(
    q_components: Query<(&T, &Handle<T>), Changed<T>>,
    mut assets: ResMut<Assets<T>>,
) {
    for (comp, handle) in q_components.iter() {
        if let Some(asset) = assets.get_mut(handle) {
            *asset = comp.clone();
        }
    }
}
