use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::render_graph::*;
use bevy::render::render_resource::binding_types::texture_2d;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::ViewTarget;
use bevy::render::{Render, RenderApp, RenderSet};
use binding_types::sampler;

use crate::extract::ExtractTexture;
use crate::radiance_cascades::RadianceCascadesTextures;
use crate::FlatlandGi;

pub struct MipmapPlugin;

impl Plugin for MipmapPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<MipmapPipelineNode>>(Core2d, FlatlandGi::Mipmap)
            .add_systems(
                Render,
                prepare_mipmap_textures
                    .in_set(RenderSet::PrepareResources)
                    .after(super::radiance_cascades::calculate_cascade_count),
            );
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<MipmapPipeline>();
        }
    }
}

fn prepare_mipmap_textures(
    mut commands: Commands,
    q_views: Query<(&ViewTarget, &MipmapConfig, Entity)>,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
) {
    for (view, config, entity) in q_views.iter() {
        let size = view.main_texture().size();

        let cached_tex = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some("rc_mipmap_texture"),
                size,
                mip_level_count: config.mip_count,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::STORAGE_BINDING
                    | TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_DST,
                view_formats: &[],
            },
        );

        let views = (0..config.mip_count)
            .map(|mip| {
                cached_tex.texture.create_view(&TextureViewDescriptor {
                    label: Some(&format!("rc_mip{mip}_view")),
                    format: None,
                    dimension: None,
                    aspect: TextureAspect::All,
                    base_mip_level: mip,
                    mip_level_count: Some(1),
                    base_array_layer: 0,
                    array_layer_count: None,
                })
            })
            .collect::<Vec<_>>();

        commands
            .entity(entity)
            .insert(MipmapTexture { cached_tex, views });
    }
}

#[derive(Component)]
pub struct MipmapTexture {
    pub cached_tex: CachedTexture,
    pub views: Vec<TextureView>,
}

#[derive(Default)]
pub struct MipmapPipelineNode;

impl ViewNode for MipmapPipelineNode {
    type ViewQuery = (
        &'static ExtractTexture,
        &'static MipmapConfig,
        &'static MipmapTexture,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (extract_tex, config, mipmap_tex): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        // Get pipeline from the cache.
        let mipmap_pipeline = world.resource::<MipmapPipeline>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(mipmap_pipeline.pipeline_id) else {
            return Ok(());
        };

        let extract_texture = &extract_tex.texture;
        render_context.command_encoder().copy_texture_to_texture(
            extract_texture.as_image_copy(),
            mipmap_tex.cached_tex.texture.as_image_copy(),
            extract_texture.size(),
        );

        for target_mip in 1..config.mip_count as usize {
            let bind_group = render_context.render_device().create_bind_group(
                "rc_mipmap_bind_group",
                &mipmap_pipeline.layout,
                &BindGroupEntries::sequential((
                    &mipmap_tex.views[target_mip - 1],
                    &mipmap_pipeline.sampler,
                )),
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("rc_mipmap_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &mipmap_tex.views[target_mip],
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_render_pipeline(pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MipmapConfig {
    pub mip_count: u32,
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct MipmapPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for MipmapPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(
            "rc_mipmap_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // Screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // Screen texture sampler
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor {
            label: Some("rc_mipmap_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let shader = world.load_asset("shaders/radiance_cascades/blit.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("rc_mipmap_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: RadianceCascadesTextures::CASCADE_FORMAT,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
