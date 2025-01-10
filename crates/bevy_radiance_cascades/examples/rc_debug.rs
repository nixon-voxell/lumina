use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::binding_types::texture_2d;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::view::ViewTarget;
use bevy::render::{render_graph::*, RenderSet};
use bevy::render::{Render, RenderApp};
use bevy::sprite::Mesh2dHandle;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_radiance_cascades::prelude::*;
use bevy_radiance_cascades::FlatlandGiPlugin;
use binding_types::sampler;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FlatlandGiPlugin)
        .add_plugins(DebugPipelinePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (config_update, marker_update))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // clear_color: Color::Srgba(Srgba::hex("19181A").unwrap().with_alpha(0.0)).into(),
                clear_color: ClearColorConfig::Custom(Color::NONE),
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                near: -500.0,
                far: 500.0,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: 1280.0,
                    max_height: 720.0,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        RadianceCascadesConfig::default(),
    ));

    const COUNT: usize = 2;
    const SPACING: f32 = 250.0;
    const OFFSET: Vec3 = Vec3::new(
        (COUNT as f32) * 0.5 * SPACING - SPACING * 0.5,
        (COUNT as f32) * 0.5 * SPACING - SPACING * 0.5,
        0.0,
    );

    for y in 0..COUNT {
        for x in 0..COUNT {
            commands.spawn((ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(15.0))),
                material: materials.add(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
                transform: Transform::from_translation(
                    Vec3::new((x as f32) * SPACING, (y as f32) * SPACING, 0.0) - OFFSET,
                ),
                ..default()
            },));
        }
    }

    commands.spawn((
        ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
            material: materials.add(Color::linear_rgba(1.7, 1.7, 1.7, 1.0)),
            // material: materials.add(Color::linear_rgba(2.0, 2.0, 2.0, 1.0)),
            ..default()
        },
        // Marker,
    ));
}

#[derive(Component)]
struct Marker;

fn config_update(
    mut q_config: Query<&mut RadianceCascadesConfig>,
    kbd_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut config) = q_config.get_single_mut() else {
        return;
    };

    let speed = match kbd_input.pressed(KeyCode::ShiftLeft) {
        true => 6.0,
        false => 3.0,
    };

    let offset = time.delta_seconds() * speed;
    if kbd_input.pressed(KeyCode::ArrowUp) {
        let interval = config.interval();
        config.set_interval(interval + offset);
    } else if kbd_input.pressed(KeyCode::ArrowDown) {
        let interval = config.interval();
        config.set_interval(interval - offset);
    }
}

fn marker_update(
    mut q_markers: Query<&mut Transform, With<Marker>>,
    q_windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = q_windows.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    for mut transform in q_markers.iter_mut() {
        if let Some(position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            transform.translation = position.extend(0.0);
        }
    }
}

pub struct DebugPipelinePlugin;

impl Plugin for DebugPipelinePlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<DebugPipelineNode>>(Core2d, DebugPipelineLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::ContrastAdaptiveSharpening,
                    DebugPipelineLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            )
            .add_systems(
                Render,
                // (|mut commands: Commands,
                //   q_textures: Query<(
                //     &bevy_radiance_cascades::mipmap::MipmapTexture,
                //     &MipmapConfig,
                //     Entity,
                // )>| {
                //     for (tex, config, entity) in q_textures.iter() {
                //         commands.entity(entity).insert(DebugTexture(
                //             tex.cached_tex.texture.create_view(&TextureViewDescriptor {
                //                 label: Some("debug_view"),
                //                 format: None,
                //                 dimension: None,
                //                 aspect: TextureAspect::All,
                //                 base_mip_level: 0,
                //                 mip_level_count: Some(config.mip_count),
                //                 base_array_layer: 0,
                //                 array_layer_count: None,
                //             }),
                //         ));
                //     }
                // })
                (|mut commands: Commands,
                  q_textures: Query<(
                    &bevy_radiance_cascades::radiance_cascades::RadianceCascadesTextures,
                    Entity,
                )>| {
                    for (tex, entity) in q_textures.iter() {
                        commands.entity(entity).insert(DebugTexture(
                            tex.converge_tex
                                // tex.main_texture()
                                .texture
                                .create_view(&TextureViewDescriptor {
                                    label: Some("debug_view"),
                                    format: None,
                                    dimension: None,
                                    aspect: TextureAspect::All,
                                    base_mip_level: 0,
                                    mip_level_count: Some(1),
                                    base_array_layer: 0,
                                    array_layer_count: None,
                                }),
                        ));
                    }
                })
                .after(RenderSet::PrepareResources),
            );
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<DebugPipeline>();
    }
}

#[derive(Component)]
pub struct DebugTexture(TextureView);

#[derive(RenderLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DebugPipelineLabel;

#[derive(Default)]
pub struct DebugPipelineNode;

impl ViewNode for DebugPipelineNode {
    type ViewQuery = (&'static ViewTarget, &'static DebugTexture);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view, texture): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        // Get pipeline from the cache.
        let debug_pipeline = world.resource::<DebugPipeline>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(debug_pipeline.pipeline_id) else {
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            "debug_bind_group",
            &debug_pipeline.layout,
            &BindGroupEntries::sequential((&texture.0, &debug_pipeline.sampler)),
        );

        let post_process = view.post_process_write();

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("debug_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // We need to specify the post process destination view here
                // to make sure we write to the appropriate texture.
                view: post_process.destination,
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

        Ok(())
    }
}

#[derive(Resource)]
pub struct DebugPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for DebugPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(
            "debug_layout",
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
            label: Some("debug_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        // Get the shader handle
        let shader = world.load_asset("shaders/radiance_cascades/debug.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("debug_pipeline".into()),
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
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: default(),
                depth_stencil: None,
                multisample: default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
