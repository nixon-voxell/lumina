use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::diagnostic::RecordDiagnostics;
use bevy::render::render_graph::*;
use bevy::render::render_phase::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::ViewTarget;
use bevy::render::{Render, RenderApp, RenderSet};

use crate::prelude::RadianceCascadesConfig;
use crate::radiance_cascades::RadianceCascadesTextures;
use crate::{FlatlandGi, NoRadiance};

#[derive(Default)]
pub struct ExtractPlugin;

impl Plugin for ExtractPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<ExtractNode>>(Core2d, FlatlandGi::Extract)
            .add_systems(
                Render,
                prepare_extract_texture.in_set(RenderSet::PrepareResources),
            );
    }
}

#[derive(Component, Deref)]
pub struct ExtractTexture(CachedTexture);

pub(super) fn prepare_extract_texture(
    mut commands: Commands,
    q_views: Query<(Entity, &ViewTarget, &RadianceCascadesConfig)>,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
) {
    for (entity, view, config) in q_views.iter() {
        let divider = config.texture_resolution as u32;
        let mut size = view.main_texture().size();

        size.width = ((size.width / 2) * 2) / divider;
        size.height = ((size.height / 2) * 2) / divider;

        let texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some("rc_extract_texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: RadianceCascadesTextures::CASCADE_FORMAT,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
        );

        commands.entity(entity).insert(ExtractTexture(texture));
    }
}

#[derive(Default)]
pub struct ExtractNode;

impl ViewNode for ExtractNode {
    type ViewQuery = (&'static ExtractedCamera, &'static ExtractTexture);

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (camera, texture): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let Some(phases) = world.get_resource::<ViewSortedRenderPhases<Transparent2d>>() else {
            return Ok(());
        };

        let view_entity = graph.view_entity();
        let Some(phase) = phases.get(&view_entity) else {
            return Ok(());
        };

        let mut extract_phase = SortedRenderPhase::default();
        for item in phase.items.iter() {
            let entity_ref = world.entity(item.entity);
            if entity_ref.contains::<NoRadiance>() == false {
                extract_phase.add(Transparent2d {
                    sort_key: item.sort_key,
                    entity: item.entity,
                    pipeline: item.pipeline,
                    draw_function: item.draw_function,
                    batch_range: item.batch_range.clone(),
                    extra_index: item.extra_index,
                });
            }
        }

        let diagnostics = render_context.diagnostic_recorder();

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("rc_extract_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture.default_view,
                resolve_target: None,
                ops: default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let pass_span = diagnostics.pass_span(&mut render_pass, "flatland_gi_extract");

        if let Some(viewport) = camera.viewport.as_ref() {
            render_pass.set_camera_viewport(viewport);
        }

        if extract_phase.items.is_empty() == false {
            extract_phase.render(&mut render_pass, world, view_entity);
        }

        pass_span.end(&mut render_pass);

        Ok(())
    }
}
