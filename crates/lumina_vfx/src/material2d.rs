use std::hash::Hash;

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dKey, Material2dPlugin};
use lumina_common::prelude::*;

use crate::main_prepass::{MainPrepassComponentPlugin, PrepassComponent};
use crate::BLEND_ADD;

pub(super) struct Material2dVfxPlugin;

impl Plugin for Material2dVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            material_from_component_plugin::<BoosterMaterial>,
            material_from_component_plugin::<PortalMaterial>,
            material_from_component_plugin::<CaveFloorMaterial>,
            material_from_component_plugin::<HealAbilityMaterial>,
            MainPrepassComponentPlugin::<HealAbilityMaterial>::default(),
        ));
    }
}

#[derive(Component, Reflect, Asset, AsBindGroup, Debug, Clone)]
#[reflect(Component)]
pub struct HealAbilityMaterial {
    #[uniform(0)]
    pub color0: LinearRgba,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(2)]
    pub time: f32,
    #[reflect(ignore)]
    #[texture(3)]
    #[sampler(4)]
    pub screen_texture: Handle<Image>,
    #[reflect(ignore)]
    #[uniform(5)]
    pub camera_scale: f32,
}

impl PrepassComponent for HealAbilityMaterial {
    fn image_mut(&mut self) -> &mut Handle<Image> {
        &mut self.screen_texture
    }

    fn camera_scale_mut(&mut self) -> &mut f32 {
        &mut self.camera_scale
    }
}

impl Material2d for HealAbilityMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vfx/heal_ability.wgsl".into()
    }
}

#[derive(Component, Reflect, Asset, AsBindGroup, Debug, Clone)]
#[reflect(Component)]
pub struct PortalMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
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
pub struct CaveFloorMaterial {
    #[uniform(0)]
    pub color0: LinearRgba,
    #[uniform(1)]
    pub color1: LinearRgba,
}

impl Material2d for CaveFloorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vfx/cave_floor.wgsl".into()
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
