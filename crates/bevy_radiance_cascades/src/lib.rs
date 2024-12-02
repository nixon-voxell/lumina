use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::prelude::*;
use bevy::render::render_graph::{RenderGraphApp, RenderLabel};
use bevy::render::RenderApp;

pub mod math_util;
pub mod mipmap;
pub mod radiance_cascades;
// pub mod radiance_mipmap;

pub mod prelude {
    pub use super::mipmap::MipmapConfig;
    pub use super::radiance_cascades::RadianceCascadesConfig;
}

pub struct FlatlandGiPlugin;

impl Plugin for FlatlandGiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off).add_plugins((
            mipmap::MipmapPlugin,
            radiance_cascades::RadianceCascadesPlugin,
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::MainTransparentPass,
                FlatlandGi::Mipmap,
                FlatlandGi::Radiance,
                Node2d::EndMainPass,
            ),
        );
    }
}

#[derive(RenderLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FlatlandGi {
    /// Main texture mipmap.
    Mipmap,
    /// Calculate global radiance.
    Radiance,
}
