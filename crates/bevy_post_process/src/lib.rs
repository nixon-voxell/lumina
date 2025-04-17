use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::render_graph::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::RenderApp;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(Core2d, PostProcessLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    PostProcessLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = ();

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        view_query: QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}
