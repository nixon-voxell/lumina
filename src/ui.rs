use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_vello::VelloPlugin;
use bevy_vello_graphics::VelloGraphicsPlugin;
use velyst::{prelude::*, typst_element::prelude::*, VelystPlugin};

pub mod effector_popup;
pub mod main_window;
pub mod perf_metrics;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VelloPlugin {
                canvas_render_layers: RenderLayers::from_layers(&[0, 1]),
                ..default()
            },
            VelloGraphicsPlugin,
            VelystPlugin::default(),
        ));

        app.add_plugins((
            main_window::MainWindowUiPlugin,
            perf_metrics::PerfMetricsUiPlugin,
            effector_popup::EffectorPopupUiPlugin,
        ))
        .add_systems(Startup, spawn_ui_camera);
    }
}

/// Spawn camera specifically only for Ui rendering (render layer 1).
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Ui Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::NONE.into(),
                hdr: true,
                order: 1,
                ..default()
            },
            ..default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        RenderLayers::layer(1),
    ));
}

// Helper for typst func interactions.

/// Compact query parameter for getting labeled interactions.
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

pub fn interactable_func<F: InteractableFunc>(
    q_interactions: Query<(&Interaction, &TypstLabel)>,
    mut func: ResMut<F>,
    time: Res<Time>,
    mut last_hovered: Local<Option<TypLabel>>,
    mut hovered_animation: Local<f32>,
) {
    let mut hovered_button = None;
    for (interaction, label) in q_interactions.iter() {
        if *interaction == Interaction::Hovered {
            hovered_button = Some(**label);
        }
    }

    if hovered_button != *last_hovered {
        *hovered_animation = 0.0;
        *last_hovered = hovered_button;
    }

    const SPEED: f32 = 6.0;
    // Clamp at 1.0
    *hovered_animation = f32::min(*hovered_animation + time.delta_seconds() * SPEED, 1.0);
    func.hovered_button(hovered_button, *hovered_animation as f64);
}

pub trait InteractableFunc: TypstFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64);
}

// Helper to show/hide typst functions.

pub fn can_show_content<F: TypstFunc>(show: Res<CanShowContent<F>>) -> bool {
    **show
}

#[derive(Resource, Deref, DerefMut)]
pub struct CanShowContent<F: TypstFunc>(#[deref] bool, PhantomData<F>);

impl<F: TypstFunc> Default for CanShowContent<F> {
    fn default() -> Self {
        Self(false, PhantomData)
    }
}
