use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_vello_graphics::{bevy_vello::VelloPlugin, prelude::*};
use velyst::VelystPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VelloPlugin {
                canvas_render_layers: RenderLayers::layer(1),
                ..default()
            },
            VelloGraphicsPlugin,
            VelystPlugin::default(),
        ));
    }
}
