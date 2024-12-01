use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::prelude::*;
use bevy::render::render_graph::{RenderGraphApp, RenderLabel};
use bevy::render::{Render, RenderApp};

pub mod generate_mipmap;
pub mod math_util;
pub mod radiance_cascades;

pub mod prelude {
    pub use super::generate_mipmap::MipmapConfig;
}

pub struct FlatlandGiPlugin;

impl Plugin for FlatlandGiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off).add_plugins((
            generate_mipmap::GenerateMipmapPlugin,
            // radiance_cascades::RadianceCascadesPlugin,
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::MainTransparentPass,
                    FlatlandGi::Mipmap,
                    // FlatlandGi::Radiance,
                    Node2d::EndMainPass,
                ),
            )
            .configure_sets(Render, (FlatlandGi::Mipmap, FlatlandGi::Radiance).chain());
    }
}

#[derive(RenderLabel, SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FlatlandGi {
    Mipmap,
    Radiance,
}
