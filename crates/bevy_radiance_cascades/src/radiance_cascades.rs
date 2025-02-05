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
use bevy::render::Render;
use bevy::render::{RenderApp, RenderSet};
use binding_types::sampler;

use crate::extract::{prepare_extract_texture, ExtractTexture};
use crate::math_util::{batch_count, cascade_count};
use crate::mipmap::MipmapTexture;
use crate::prelude::MipmapConfig;
use crate::FlatlandGi;

/// MAX to 11 because that is the limit of mipmaps.
pub const MAX_CASCADE_COUNT: usize = 11;

pub struct RadianceCascadesPlugin;

impl Plugin for RadianceCascadesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<RadianceCascadesConfig>::default());

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
                    (
                        calculate_cascade_count.after(prepare_extract_texture),
                        (prepare_rc_textures, prepare_rc_buffers),
                    )
                        .chain()
                        .in_set(RenderSet::PrepareResources),
                    prepare_rc_bind_groups.in_set(RenderSet::PrepareBindGroups),
                ),
            );

        app.register_type::<RadianceCascadesConfig>();
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<RadianceCascadesPipeline>();
        }
    }
}

pub(super) fn calculate_cascade_count(
    mut commands: Commands,
    q_views: Query<(&ExtractTexture, &RadianceCascadesConfig, Entity)>,
) {
    for (extract_tex, rc_config, entity) in q_views.iter() {
        let size = extract_tex.texture.size();
        // Use diagonal length as the max length. (A^2 + B^2 = C^2)
        let max_length = f32::sqrt((size.width * size.width + size.height * size.height) as f32);

        let mut cascade_count = cascade_count(max_length, rc_config.interval0);
        cascade_count = usize::min(cascade_count, MAX_CASCADE_COUNT);

        commands.entity(entity).insert((
            CascadeCount(cascade_count),
            MipmapConfig {
                mip_count: 1 + cascade_count as u32,
            },
        ));
    }
}

fn prepare_rc_textures(
    mut commands: Commands,
    q_views: Query<(&ExtractTexture, &CascadeCount, Entity)>,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
) {
    for (extract_tex, cascade_count, entity) in q_views.iter() {
        let size = extract_tex.texture.size();

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

        let cached_tex0 = texture_cache.get(&render_device, rc_texture_desc("rc_texture0"));
        let cached_tex1 = texture_cache.get(&render_device, rc_texture_desc("rc_texture1"));

        let converge_tex = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some("rc_converge_texture"),
                size: size.mip_level_size(1, TextureDimension::D2),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: RadianceCascadesTextures::CASCADE_FORMAT,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::STORAGE_BINDING,
                view_formats: &[],
            },
        );

        commands.entity(entity).insert(RadianceCascadesTextures {
            cached_tex0,
            cached_tex1,
            converge_tex,
            is_texture0: cascade_count.0 % 2 != 0,
        });
    }
}

fn prepare_rc_buffers(
    mut commands: Commands,
    q_configs: Query<(&RadianceCascadesConfig, &CascadeCount, Entity)>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (config, cascade_count, entity) in q_configs.iter() {
        let mut num_cascades = UniformBuffer::default();
        let mut probe_buffers = DynamicUniformBuffer::default();
        let mut c0_probe_width = UniformBuffer::default();
        probe_buffers.set_label(Some("rc_probe_buffers"));

        let cascade_count = cascade_count.0;
        num_cascades.set(cascade_count as u32);
        let mut probe_buffer_offsets = Vec::with_capacity(cascade_count);

        for c in 0..cascade_count {
            let width = 1 << (c as u32 + config.resolution_factor);
            let start = config.interval0 * (1.0 - f32::powi(4.0, c as i32)) / -3.0;
            let range = config.interval0 * f32::powi(4.0, c as i32);

            let probe = Probe {
                cascade_index: c as u32,
                width,
                start,
                range,
            };

            let offset = probe_buffers.push(&probe);
            probe_buffer_offsets.push(offset);
        }

        let c0_width = 1 << config.resolution_factor;
        c0_probe_width.set(c0_width);

        num_cascades.write_buffer(&render_device, &render_queue);
        probe_buffers.write_buffer(&render_device, &render_queue);
        c0_probe_width.write_buffer(&render_device, &render_queue);

        commands.entity(entity).insert(RadianceCascadesBuffer {
            num_cascades,
            probe_buffers,
            probe_buffer_offsets,
            c0_probe_width,
        });
    }
}

fn prepare_rc_bind_groups(
    mut commands: Commands,
    q_views: Query<(
        &MipmapTexture,
        &RadianceCascadesTextures,
        &RadianceCascadesBuffer,
        Entity,
    )>,
    render_device: Res<RenderDevice>,
    pipeline: Res<RadianceCascadesPipeline>,
) {
    for (mipmap_texture, rc_textures, buffer, entity) in q_views.iter() {
        let main_bind_group = render_device.create_bind_group(
            "rc_main_bind_group ",
            &pipeline.main_layout,
            &BindGroupEntries::sequential((
                // Num cascades
                &buffer.num_cascades,
                // Probe
                &buffer.probe_buffers,
                // Main texture
                &mipmap_texture
                    .cached_tex
                    .texture
                    .create_view(&TextureViewDescriptor {
                        label: Some("rc_main"),
                        format: None,
                        dimension: None,
                        aspect: TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: Some(mipmap_texture.cached_tex.texture.mip_level_count()),
                        base_array_layer: 0,
                        array_layer_count: None,
                    }),
                // Main sampler
                &render_device.create_sampler(&SamplerDescriptor {
                    label: Some("rc_main_sampler"),
                    address_mode_u: AddressMode::ClampToEdge,
                    address_mode_v: AddressMode::ClampToEdge,
                    address_mode_w: AddressMode::ClampToEdge,
                    mag_filter: FilterMode::Nearest,
                    min_filter: FilterMode::Nearest,
                    mipmap_filter: FilterMode::Nearest,
                    ..Default::default()
                }),
            )),
        );

        let cascade01_bind_group = render_device.create_bind_group(
            "rc_cascade01_bind_group",
            &pipeline.cascade_layout,
            &BindGroupEntries::sequential((
                &rc_textures.cached_tex0.default_view,
                &rc_textures.cached_tex1.default_view,
            )),
        );

        let cascade10_bind_group = render_device.create_bind_group(
            "rc_cascade10_bind_group",
            &pipeline.cascade_layout,
            &BindGroupEntries::sequential((
                &rc_textures.cached_tex1.default_view,
                &rc_textures.cached_tex0.default_view,
            )),
        );

        let converge_bind_group = render_device.create_bind_group(
            "rc_converge_bind_group",
            &pipeline.converge_layout,
            &BindGroupEntries::sequential((
                &buffer.c0_probe_width,
                &rc_textures.main_texture().default_view,
                &rc_textures.converge_tex.default_view,
            )),
        );

        commands.entity(entity).insert(RadianceCascadesBindGroups {
            main_bind_group,
            cascade01_bind_group,
            cascade10_bind_group,
            converge_bind_group,
        });
    }
}

/// Adding this to [bevy::prelude::Camera2d] will enable Radiance Cascades GI.
#[derive(ExtractComponent, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
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

    pub fn resolution_factor(&self) -> u32 {
        self.resolution_factor
    }

    pub fn interval(&self) -> f32 {
        self.interval0
    }
}

impl Default for RadianceCascadesConfig {
    fn default() -> Self {
        Self {
            resolution_factor: 1,
            interval0: 4.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CascadeCount(usize);

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
    num_cascades: UniformBuffer<u32>,
    probe_buffers: DynamicUniformBuffer<Probe>,
    probe_buffer_offsets: Vec<u32>,
    c0_probe_width: UniformBuffer<u32>,
}

#[derive(Component)]
pub struct RadianceCascadesTextures {
    pub cached_tex0: CachedTexture,
    pub cached_tex1: CachedTexture,
    pub converge_tex: CachedTexture,
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
    main_bind_group: BindGroup,
    cascade01_bind_group: BindGroup,
    cascade10_bind_group: BindGroup,
    converge_bind_group: BindGroup,
}

#[derive(Default)]
pub struct RadianceCascadesNode;

impl ViewNode for RadianceCascadesNode {
    type ViewQuery = (
        &'static RadianceCascadesTextures,
        &'static CascadeCount,
        &'static RadianceCascadesBindGroups,
        &'static RadianceCascadesBuffer,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (rc_tex, cascade_count, bind_groups, buffer): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let pipeline = world.resource::<RadianceCascadesPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache.
        let (Some(no_merge_pipeline), Some(merge_pipeline), Some(converge_pipeline)) = (
            pipeline_cache.get_compute_pipeline(pipeline.no_merge_pipeline),
            pipeline_cache.get_compute_pipeline(pipeline.merge_pipeline),
            pipeline_cache.get_compute_pipeline(pipeline.converge_pipeline),
        ) else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("rc_pass_group");

        let screen_size = rc_tex.converge_tex.texture.size();
        let rc_size = rc_tex.cached_tex0.texture.size();
        let rc_workgroup_size = batch_count(
            UVec3::new(rc_size.width, rc_size.height, 1),
            UVec3::new(8, 8, 1),
        );
        let screen_workgroup_size = batch_count(
            UVec3::new(screen_size.width, screen_size.height, 1),
            UVec3::new(8, 8, 1),
        );

        // Radiance cascades pass.
        {
            let mut rc_compute_pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("rc_pass"),
                        timestamp_writes: None,
                    });

            let cascade_count = cascade_count.0 - 1;
            // First cascade does not require any merging.
            rc_compute_pass.set_pipeline(no_merge_pipeline);

            // Set bind groups.
            rc_compute_pass.set_bind_group(
                0,
                &bind_groups.main_bind_group,
                &[buffer.probe_buffer_offsets[cascade_count]],
            );
            rc_compute_pass.set_bind_group(1, &bind_groups.cascade10_bind_group, &[]);

            // Dispatch compute shader.
            rc_compute_pass.dispatch_workgroups(
                rc_workgroup_size.x,
                rc_workgroup_size.y,
                rc_workgroup_size.z,
            );

            // Merging is required after the first cascade.
            rc_compute_pass.set_pipeline(merge_pipeline);

            for c in 0..cascade_count {
                let offset_index = cascade_count - c - 1;

                // Set bind groups
                let cascade_bind_group = match c % 2 == 0 {
                    true => &bind_groups.cascade01_bind_group,
                    false => &bind_groups.cascade10_bind_group,
                };
                rc_compute_pass.set_bind_group(
                    0,
                    &bind_groups.main_bind_group,
                    &[buffer.probe_buffer_offsets[offset_index]],
                );
                rc_compute_pass.set_bind_group(1, cascade_bind_group, &[]);

                // Dispatch compute shader
                rc_compute_pass.dispatch_workgroups(
                    rc_workgroup_size.x,
                    rc_workgroup_size.y,
                    rc_workgroup_size.z,
                );
            }

            // Converge cascade 0 radiance into screen size.
            rc_compute_pass.set_pipeline(converge_pipeline);

            // Set bind groups.
            rc_compute_pass.set_bind_group(0, &bind_groups.converge_bind_group, &[]);

            // Dispatch compute shader.
            rc_compute_pass.dispatch_workgroups(
                screen_workgroup_size.x,
                screen_workgroup_size.y,
                screen_workgroup_size.z,
            );
        }

        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
struct RadianceCascadesPipeline {
    main_layout: BindGroupLayout,
    cascade_layout: BindGroupLayout,
    converge_layout: BindGroupLayout,
    no_merge_pipeline: CachedComputePipelineId,
    merge_pipeline: CachedComputePipelineId,
    converge_pipeline: CachedComputePipelineId,
}

impl FromWorld for RadianceCascadesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // Shader.
        let rc_shader = world.load_asset("shaders/radiance_cascades/radiance_cascades.wgsl");
        let converge_shader = world.load_asset("shaders/radiance_cascades/converge.wgsl");

        // Bind group layout.
        let main_layout = render_device.create_bind_group_layout(
            "rc_main_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    // Num cascadesee
                    uniform_buffer::<u32>(false),
                    // Probe
                    uniform_buffer::<Probe>(true),
                    // Main texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // Main texture sampler
                    sampler(SamplerBindingType::NonFiltering),
                ),
            ),
        );

        let cascade_layout = render_device.create_bind_group_layout(
            "rc_cascade_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
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

        let converge_layout = render_device.create_bind_group_layout(
            "converge_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    // Probe width
                    uniform_buffer::<u32>(false),
                    // Cascade 0 texture
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    // Converge texture
                    texture_storage_2d(
                        RadianceCascadesTextures::CASCADE_FORMAT,
                        StorageTextureAccess::WriteOnly,
                    ),
                ),
            ),
        );

        // Pipeline
        let no_merge_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("rc_no_merge_pipeline".into()),
            layout: vec![main_layout.clone(), cascade_layout.clone()],
            shader: rc_shader.clone(),
            shader_defs: vec![],
            entry_point: "radiance_cascades".into(),
            push_constant_ranges: vec![],
        });

        let merge_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("rc_merge_pipeline".into()),
            layout: vec![main_layout.clone(), cascade_layout.clone()],
            shader: rc_shader,
            shader_defs: vec!["MERGE".into()],
            entry_point: "radiance_cascades".into(),
            push_constant_ranges: vec![],
        });

        let converge_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("rc_converge_pipeline".into()),
            layout: vec![converge_layout.clone()],
            shader: converge_shader,
            shader_defs: vec![],
            entry_point: "converge".into(),
            push_constant_ranges: vec![],
        });

        Self {
            main_layout,
            cascade_layout,
            converge_layout,
            no_merge_pipeline,
            merge_pipeline,
            converge_pipeline,
        }
    }
}
