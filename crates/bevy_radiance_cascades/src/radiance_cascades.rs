use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_graph::*;
use bevy::render::render_resource::binding_types::{
    texture_2d, texture_storage_2d, uniform_buffer,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::ViewTarget;
use bevy::render::Render;
use bevy::render::{RenderApp, RenderSet};

use crate::generate_mipmap::MipmapTexture;
use crate::math_util::{batch_count, cascade_count};
use crate::prelude::MipmapConfig;
use crate::FlatlandGi;

pub const MAX_CASCADE_COUNT: usize = 16;

pub struct RadianceCascadesPlugin;

impl Plugin for RadianceCascadesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<RadianceCascadesConfig>::default())
            .insert_resource(Msaa::Off);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<RadianceCascadesNode>>(
                Core2d,
                FlatlandGi::Radiance,
            )
            .add_systems(
                Render,
                (
                    calculate_cascade_count.in_set(RenderSet::PrepareResources),
                    (prepare_rc_textures, prepare_rc_buffers)
                        .in_set(RenderSet::PrepareResources)
                        .after(calculate_cascade_count),
                    prepare_rc_bind_groups.in_set(RenderSet::PrepareBindGroups),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<RadianceCascadesPipeline>();
    }
}

#[derive(Default)]
pub struct RadianceCascadesNode;

impl ViewNode for RadianceCascadesNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static RadianceCascadesBindGroups,
        &'static RadianceCascadesCount,
        &'static RadianceCascadesBuffer,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view, bind_groups, cascade_count, buffer): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let pipeline = world.resource::<RadianceCascadesPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let (Some(rc_no_merge_pipeline), Some(rc_merge_pipeline)) = (
            pipeline_cache.get_compute_pipeline(pipeline.rc_no_merge_pipeline),
            pipeline_cache.get_compute_pipeline(pipeline.rc_merge_pipeline),
        ) else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("rc_pass_group");

        let size = view.main_texture().size();
        let workgroup_size =
            batch_count(UVec3::new(size.width, size.height, 1), UVec3::new(8, 8, 1));

        {
            let mut rc_compute_pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("rc_pass"),
                        timestamp_writes: None,
                    });

            let cascade_count = cascade_count.0 - 1;
            // First cascade does not require any merging
            rc_compute_pass.set_pipeline(rc_no_merge_pipeline);
            // Set bind groups
            rc_compute_pass.set_bind_group(
                0,
                &bind_groups.rc_10_bind_group,
                &[buffer.probe_buffer_offsets[cascade_count]],
            );

            // Dispatch compute shader
            rc_compute_pass.dispatch_workgroups(
                workgroup_size.x,
                workgroup_size.y,
                workgroup_size.z,
            );

            // Merging is required after the first cascade
            rc_compute_pass.set_pipeline(rc_merge_pipeline);

            for c in 0..cascade_count {
                let offset_index = cascade_count - c - 1;

                // Set bind groups
                let rc_bind_group = match c % 2 == 0 {
                    true => &bind_groups.rc_01_bind_group,
                    false => &bind_groups.rc_10_bind_group,
                };
                rc_compute_pass.set_bind_group(
                    0,
                    rc_bind_group,
                    &[buffer.probe_buffer_offsets[offset_index]],
                );

                // Dispatch compute shader
                rc_compute_pass.dispatch_workgroups(
                    workgroup_size.x,
                    workgroup_size.y,
                    workgroup_size.z,
                );
            }
        }

        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
struct RadianceCascadesPipeline {
    rc_bind_group_layout: BindGroupLayout,
    rc_no_merge_pipeline: CachedComputePipelineId,
    rc_merge_pipeline: CachedComputePipelineId,
}

impl FromWorld for RadianceCascadesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // Shader
        let rc_shader = world.load_asset("shaders/radiance_cascades/radiance_cascades.wgsl");

        // Bind group layout
        let rc_bind_group_layout = render_device.create_bind_group_layout(
            "rc_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    // Probe width
                    uniform_buffer::<Probe>(true),
                    // Main texture
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    // Cascade n+1 texture (source)
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    // Cascade n texture (destination)
                    texture_storage_2d(
                        RadianceCascadesTextures::CASCADE_FORMAT,
                        StorageTextureAccess::WriteOnly,
                    ),
                ),
            ),
        );

        // Pipeline
        let rc_no_merge_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("rc_no_merge_pipeline".into()),
                layout: vec![rc_bind_group_layout.clone()],
                shader: rc_shader.clone(),
                shader_defs: vec![],
                entry_point: "rc".into(),
                push_constant_ranges: vec![],
            });

        let rc_merge_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("rc_merge_pipeline".into()),
            layout: vec![rc_bind_group_layout.clone()],
            shader: rc_shader,
            shader_defs: vec!["MERGE".into()],
            entry_point: "rc".into(),
            push_constant_ranges: vec![],
        });

        Self {
            rc_bind_group_layout,
            rc_no_merge_pipeline,
            rc_merge_pipeline,
        }
    }
}

fn calculate_cascade_count(
    mut commands: Commands,
    mut q_views: Query<(
        Entity,
        &ViewTarget,
        &RadianceCascadesConfig,
        &mut MipmapConfig,
    )>,
) {
    for (entity, view, rc_config, mut mipmap_config) in q_views.iter_mut() {
        let size = view.main_texture().size();
        // Use diagonal length as the max length
        let max_length = f32::sqrt((size.width * size.width + size.height * size.height) as f32);

        let mut cascade_count = cascade_count(max_length, rc_config.interval0);
        cascade_count = usize::min(cascade_count, MAX_CASCADE_COUNT);

        mipmap_config.mip_count = 2 * cascade_count as u32;

        commands
            .entity(entity)
            .insert(RadianceCascadesCount(cascade_count));
    }
}

fn prepare_rc_textures(
    mut commands: Commands,
    q_views: Query<(
        Entity,
        &ViewTarget,
        &RadianceCascadesCount,
        &RadianceCascadesConfig,
    )>,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
) {
    for (entity, view, cascade_count, cascade_config) in q_views.iter() {
        let mut size = view.main_texture().size();
        size.depth_or_array_layers = 1;

        let mut half_size = size;
        let probe_width = 1 << cascade_config.resolution_factor;
        half_size.width = (half_size.width + probe_width - 1) / probe_width;
        half_size.height = (half_size.height + probe_width - 1) / probe_width;

        let rc_texture_desc = |name: &'static str| TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: RadianceCascadesTextures::CASCADE_FORMAT,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let cached_tex0 = texture_cache.get(&render_device, rc_texture_desc("rc_0_texture"));
        let cached_tex1 = texture_cache.get(&render_device, rc_texture_desc("rc_1_texture"));

        commands.entity(entity).insert(RadianceCascadesTextures {
            cached_tex0,
            cached_tex1,
            is_texture0: cascade_count.0 % 2 != 0,
        });
    }
}

fn prepare_rc_buffers(
    mut commands: Commands,
    q_configs: Query<(Entity, &RadianceCascadesConfig, &RadianceCascadesCount)>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (entity, config, cascade_count) in q_configs.iter() {
        let mut probe_buffers = DynamicUniformBuffer::default();
        probe_buffers.set_label(Some("rc_probe_buffers"));

        let cascade_count = cascade_count.0;
        let mut probe_buffer_offsets = Vec::with_capacity(cascade_count);

        for c in 0..cascade_count {
            let width = 1 << (c as u32 + config.resolution_factor);
            let start = config.interval0 * (1.0 - f32::powi(4.0, c as i32)) / -3.0;
            let range = config.interval0 * f32::powi(4.0, c as i32);
            // TODO: calculate the number of pixels to raymarch directly here.

            let probe = Probe {
                cascade_index: c as u32,
                width,
                start,
                range,
            };

            let offset = probe_buffers.push(&probe);
            probe_buffer_offsets.push(offset);
        }

        probe_buffers.write_buffer(&render_device, &render_queue);

        commands.entity(entity).insert(RadianceCascadesBuffer {
            probe_buffers,
            probe_buffer_offsets,
        });
    }
}

fn prepare_rc_bind_groups(
    mut commands: Commands,
    q_views: Query<(
        Entity,
        &MipmapTexture,
        &RadianceCascadesTextures,
        &RadianceCascadesBuffer,
    )>,
    render_device: Res<RenderDevice>,
    pipeline: Res<RadianceCascadesPipeline>,
) {
    for (entity, mipmap_texture, rc_textures, buffer) in q_views.iter() {
        let rc_01_bind_group = render_device.create_bind_group(
            "radiance_cascade_01_bind_group",
            &pipeline.rc_bind_group_layout,
            &BindGroupEntries::sequential((
                &buffer.probe_buffers,
                // TODO: Create custom view?
                &mipmap_texture.cached_tex.default_view,
                &rc_textures.cached_tex0.default_view,
                &rc_textures.cached_tex1.default_view,
            )),
        );

        let rc_10_bind_group = render_device.create_bind_group(
            "radiance_cascade_10_bind_group",
            &pipeline.rc_bind_group_layout,
            &BindGroupEntries::sequential((
                &buffer.probe_buffers,
                // TODO: Create custom view?
                &mipmap_texture.cached_tex.default_view,
                &rc_textures.cached_tex1.default_view,
                &rc_textures.cached_tex0.default_view,
            )),
        );

        commands.entity(entity).insert(RadianceCascadesBindGroups {
            rc_01_bind_group,
            rc_10_bind_group,
        });
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct RadianceCascadesLabel;

/// Adding this to [bevy::prelude::Camera2d] will enable Radiance Cascades GI.
#[derive(ExtractComponent, Component, Clone, Copy)]
pub struct RadianceCascadesConfig {
    /// Determines the number of directions in cascade 0 (angular resolution).
    /// `angular_resolution = resolution_factor * 4`.
    resolution_factor: u32,
    /// Interval length of cascade 0 in pixel unit.
    interval0: f32,
}

impl RadianceCascadesConfig {
    /// Creates a new radiance cascades configuration with resolution
    /// factor and interval0 clamped above 1.
    pub fn new(mut resolution_factor: u32, mut interval0: f32) -> Self {
        resolution_factor = u32::max(resolution_factor, 1);
        interval0 = f32::max(interval0, 1.0);
        Self {
            resolution_factor,
            interval0,
        }
    }

    /// New config with resolution factor (clamped above 1).
    pub fn with_resolution_factor(mut self, mut resolution_factor: u32) -> Self {
        resolution_factor = u32::max(resolution_factor, 1);
        self.resolution_factor = resolution_factor;
        self
    }

    /// New config with interval length in pixel unit (clamped above 1).
    pub fn with_interval(mut self, mut interval0: f32) -> Self {
        interval0 = f32::max(interval0, 1.0);
        self.interval0 = interval0;
        self
    }

    /// Mutably set resolution factor (clamped above 1).
    pub fn set_resolution_factor(&mut self, mut resolution_factor: u32) {
        resolution_factor = u32::max(resolution_factor, 1);
        self.resolution_factor = resolution_factor;
    }

    /// Mutably set interval length in pixel unit (clamped above 1).
    pub fn set_interval(&mut self, mut interval0: f32) {
        interval0 = f32::max(interval0, 1.0);
        self.interval0 = interval0;
    }

    pub fn get_resolution_factor(&self) -> u32 {
        self.resolution_factor
    }

    pub fn get_interval(&self) -> f32 {
        self.interval0
    }
}

impl Default for RadianceCascadesConfig {
    fn default() -> Self {
        Self {
            resolution_factor: 1,
            interval0: 2.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct RadianceCascadesCount(usize);

#[derive(ShaderType, Debug, Clone, Copy)]
struct Probe {
    /// Cascade index.
    pub cascade_index: u32,
    /// Number of pixels between probes.
    pub width: u32,
    /// Staring offset.
    pub start: f32,
    /// Range of ray.
    pub range: f32,
}

#[derive(Component)]
pub struct RadianceCascadesBuffer {
    probe_buffers: DynamicUniformBuffer<Probe>,
    probe_buffer_offsets: Vec<u32>,
}

#[derive(Component)]
pub struct RadianceCascadesTextures {
    pub cached_tex0: CachedTexture,
    pub cached_tex1: CachedTexture,
    is_texture0: bool,
}

impl RadianceCascadesTextures {
    pub const CASCADE_FORMAT: TextureFormat = TextureFormat::Rgba16Float;

    pub fn main_texture(&self) -> &CachedTexture {
        match self.is_texture0 {
            true => &self.cached_tex0,
            false => &self.cached_tex1,
        }
    }
}

#[derive(Component)]
pub struct RadianceCascadesBindGroups {
    rc_01_bind_group: BindGroup,
    rc_10_bind_group: BindGroup,
}
