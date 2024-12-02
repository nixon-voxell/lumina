use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_graph::{RenderGraphApp, RenderLabel};
use bevy::render::RenderApp;

pub mod extract;
pub mod math_util;
pub mod mipmap;
pub mod radiance_cascades;

pub mod prelude {
    pub use super::mipmap::MipmapConfig;
    pub use super::radiance_cascades::RadianceCascadesConfig;
    pub use super::Radiance;
}

pub struct FlatlandGiPlugin;

impl Plugin for FlatlandGiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            extract::ExtractPlugin::<ColorMaterial>::default(),
            mipmap::MipmapPlugin,
            radiance_cascades::RadianceCascadesPlugin,
        ))
        .add_plugins(ExtractComponentPlugin::<Radiance>::default());
        // TODO: REMOVE THIS?
        // .insert_resource(Msaa::Off);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::MainTransparentPass,
                FlatlandGi::Extract,
                FlatlandGi::Mipmap,
                FlatlandGi::Radiance,
                Node2d::EndMainPass,
            ),
        );
    }
}

#[derive(RenderLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FlatlandGi {
    /// Extract entities with [`Radiance`] component.
    Extract,
    /// Main texture mipmap.
    Mipmap,
    /// Calculate global radiance.
    Radiance,
    /// Composite the radiance into the main texture.
    Composite,
}

/// Marker component for renderable entities that contributes to the global radiance.
#[derive(Component, ExtractComponent, Clone, Copy)]
pub struct Radiance;
