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

// pub fn hovered<'a>(
//     mut q_interactions: impl Iterator<Item = (&'a Interaction, &'a TypstLabel)>,
//     label_str: &str,
// ) -> bool {
//     q_interactions.any(|(interaction, label)| {
//         label.as_str() == label_str && *interaction == Interaction::Hovered
//     })
// }

pub fn windowed_func<F: WindowedFunc>(
    q_window: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut func: ResMut<F>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.set_width_height(window.width() as f64, window.height() as f64);
}

pub trait WindowedFunc: TypstFunc {
    fn set_width_height(&mut self, width: f64, height: f64);
}
