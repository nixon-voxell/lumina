use std::marker::PhantomData;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::FocusPolicy;
use bevy_motiongfx::prelude::ease;
use bevy_vello::VelloPlugin;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;
use velyst::VelystPlugin;

pub mod effector_popup;
pub mod interaction;
pub mod main_window;
pub mod perf_metrics;

pub mod prelude {
    pub use crate::effector_popup::{EffectorPopupFunc, EffectorPopupUi};
    pub use crate::interaction::AppExt;
    pub use crate::main_window::{
        push_to_main_window, push_to_main_window_background, push_to_main_window_foreground,
        MainWindowFunc, MainWindowSet, MainWindowTransparency, WINDOW_FADE_DURATION,
    };
    pub use crate::perf_metrics::PerfMetricsFunc;
    pub use crate::{
        can_show_content, interactable_func, CanShowContent, InteractableFunc, InteractionQuery,
    };
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Using assets/fonts as the fonts path
        let mut fonts_path = PathBuf::from(".");
        fonts_path.push("assets");
        fonts_path.push("fonts");
        app.add_plugins((
            VelloPlugin {
                canvas_render_layers: RenderLayers::from_layers(&[0, 1]),
                ..default()
            },
            VelystPlugin::new(vec![fonts_path]),
        ));

        app.add_plugins((
            main_window::MainWindowUiPlugin,
            perf_metrics::PerfMetricsUiPlugin,
            effector_popup::EffectorPopupUiPlugin,
            interaction::InteractionPlugin,
        ))
        .add_systems(Startup, spawn_ui_camera)
        .add_systems(Update, disable_specific_interactions);
    }
}

/// Spawn camera specifically only for Ui rendering (render layer 1).
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Ui Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
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

/// Disable interactions with non-button labels.
fn disable_specific_interactions(
    mut commands: Commands,
    mut q_interactions: Query<(&TypstLabel, Entity), Added<Interaction>>,
) {
    for (label, entity) in q_interactions.iter_mut() {
        if label.resolve().starts_with("btn") == false {
            commands
                .entity(entity)
                .insert(FocusPolicy::Pass)
                .remove::<Interaction>();
        }
    }
}

pub fn interactable_func<F: InteractableFunc>(
    q_interactions: Query<(&Interaction, &TypstLabel)>,
    mut func: ResMut<F>,
    time: Res<Time>,
    mut last_hovered: Local<Option<TypLabel>>,
    mut hovered_animation: Local<f32>,
) {
    const SPEED: f32 = 2.0;

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

    // Clamp at 1.0
    *hovered_animation = f32::min(*hovered_animation + time.delta_seconds() * SPEED, 1.0);
    func.hovered_button(
        hovered_button,
        ease::cubic::ease_in_out(*hovered_animation) as f64,
    );
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

/// Compact query parameter for getting labeled interactions.
#[derive(bevy::ecs::system::SystemParam)]
pub struct InteractionQuery<'w, 's> {
    pub q_interactions:
        Query<'w, 's, (&'static Interaction, &'static TypstLabel), Changed<Interaction>>,
}

impl InteractionQuery<'_, '_> {
    pub fn pressed(&self, label_str: &str) -> bool {
        self.is_interacting(label_str, &Interaction::Pressed)
    }

    pub fn hovered(&self, label_str: &str) -> bool {
        self.is_interacting(label_str, &Interaction::Hovered)
    }

    pub fn none(&self, label_str: &str) -> bool {
        self.is_interacting(label_str, &Interaction::None)
    }

    pub fn is_interacting(&self, label_str: &str, target_interaction: &Interaction) -> bool {
        self.q_interactions.iter().any(|(interaction, label)| {
            label.resolve().as_str() == label_str && interaction == target_interaction
        })
    }
}
