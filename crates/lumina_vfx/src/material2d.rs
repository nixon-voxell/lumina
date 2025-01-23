use std::hash::Hash;

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dKey, Material2dPlugin};
use lumina_common::prelude::*;

use crate::BLEND_ADD;

pub(super) struct Material2dVfxPlugin;

impl Plugin for Material2dVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            material_from_component_plugin::<BoosterMaterial>,
            // material_from_component_plugin::<PortalMaterial>,
        ));
    }
}

#[derive(Component, Reflect, Asset, AsBindGroup, Debug, Clone)]
#[reflect(Component)]
pub struct PortalMaterial {
    #[uniform(0)]
    pub primary_color: LinearRgba,
    #[uniform(1)]
    pub secondary_color: LinearRgba,
}

impl Material2d for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vfx/portal.wgsl".into()
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

pub fn material_from_component_plugin<M: AssetFromComponent + Material2d>(app: &mut App)
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    app.add_plugins((
        AssetFromComponentPlugin::<M>::default(),
        Material2dPlugin::<M>::default(),
    ))
    .add_systems(PostUpdate, remove_color_material::<M>);
}

fn remove_color_material<M: AssetFromComponent>(
    mut commands: Commands,
    q_entities: Query<Entity, Added<M>>,
) {
    for entity in q_entities.iter() {
        commands
            .entity(entity)
            .remove::<(Handle<ColorMaterial>, Handle<StandardMaterial>)>();
    }
}
