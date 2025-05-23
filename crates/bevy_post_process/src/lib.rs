use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;
use chromatic_aberration::{ChromaticAberrationLabel, ChromaticAberrationPlugin};
use glitch::{GlitchLabel, GlitchPlugin};
use greyscale::{GreyscaleLabel, GreyscalePlugin};
use vignette::{VignetteLabel, VignettePlugin};

pub mod chromatic_aberration;
pub mod glitch;
pub mod greyscale;
pub mod vignette;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChromaticAberrationPlugin,
            GreyscalePlugin,
            GlitchPlugin,
            VignettePlugin,
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_edges(
            Core2d,
            (
                Node2d::Tonemapping,
                ChromaticAberrationLabel,
                GreyscaleLabel,
                GlitchLabel,
                VignetteLabel,
                Node2d::EndMainPassPostProcessing,
            ),
        );
    }
}
