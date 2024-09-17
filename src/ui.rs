use bevy::render::view::RenderLayers;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_vello_graphics::{bevy_vello::VelloPlugin, prelude::*};
use velyst::{prelude::*, VelystPlugin};

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

pub type InteractionQuery<'a, 'w, 's> =
    Query<'w, 's, (&'a Interaction, &'a TypstLabel), Changed<Interaction>>;

pub fn pressed<'a>(
    mut q_interactions: impl Iterator<Item = (&'a Interaction, &'a TypstLabel)>,
    label_str: &str,
) -> bool {
    q_interactions.any(|(interaction, label)| {
        label.as_str() == label_str && *interaction == Interaction::Pressed
    })
}

pub fn hovered<'a>(
    mut q_interactions: impl Iterator<Item = (&'a Interaction, &'a TypstLabel)>,
    label_str: &str,
) -> bool {
    q_interactions.any(|(interaction, label)| {
        label.as_str() == label_str && *interaction == Interaction::Hovered
    })
}

pub type WindowQuery<'a, 'w, 's> =
    Query<'w, 's, Ref<'a, Window>, (With<PrimaryWindow>, Changed<Window>)>;
