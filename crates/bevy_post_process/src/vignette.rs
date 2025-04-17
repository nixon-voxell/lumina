use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::*;
use bevy::render::render_graph::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::view::ViewTarget;
use bevy::render::RenderApp;
use binding_types::*;

pub(super) struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<VignetteConfig>::default(),
            UniformComponentPlugin::<VignetteConfig>::default(),
        ))
        .register_type::<VignetteConfig>();

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_node::<ViewNodeRunner<VignetteNode>>(Core2d, VignetteLabel);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<VignettePipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct VignetteLabel;

#[derive(Default)]
struct VignetteNode;

impl ViewNode for VignetteNode {
    type ViewQuery = (
        &'static ViewTarget,
        // Makes sure the node only runs on cameras with
        // the VignetteConfig component.
        &'static VignetteConfig,
        // As there could be multiple post processing components sent to the GPU
        // (one per camera), we need to get the index of the one that is
        // associated with the current view.
        &'static DynamicUniformIndex<VignetteConfig>,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view_target, _config, settings_index): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<VignettePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // Get the settings uniform binding
        let settings_uniforms = world.resource::<ComponentUniforms<VignetteConfig>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

/// This contains global data used by the render pipeline.
#[derive(Resource)]
struct VignettePipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for VignettePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    // The screen texture.
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture.
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect.
                    uniform_buffer::<VignetteConfig>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.load_asset("shaders/post_process/vignette.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("vignette_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rgba16Float,
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

#[derive(Component, ExtractComponent, ShaderType, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct VignetteConfig {
    pub intensity: f32,
    pub distance: f32,
    pub tint: Vec3,
}

impl Default for VignetteConfig {
    fn default() -> Self {
        Self {
            intensity: 3.0,
            distance: -0.4,
            tint: Vec3::ZERO,
        }
    }
}
