use bevy::prelude::*;
use bevy::render::render_graph::RenderLabel;
use bevy::render::{Render, RenderApp};

pub mod generate_mipmap;
pub mod math_util;
pub mod radiance_cascades;

pub mod prelude {
    pub use super::generate_mipmap::MipmapConfig;
}

pub struct RadianceCascadesPlugin;

impl Plugin for RadianceCascadesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_plugins(generate_mipmap::GenerateMipmapPlugin);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.configure_sets(
            Render,
            (RadianceCascadesPass::Mipmap, RadianceCascadesPass::Radiance).chain(),
        );
    }
}

#[derive(RenderLabel, SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RadianceCascadesPass {
    Mipmap,
    Radiance,
}
