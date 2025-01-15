use bevy::prelude::*;
use bevy_vello::prelude::*;
use velyst::prelude::*;

pub struct EffectorPopupUiPlugin;

impl Plugin for EffectorPopupUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<EffectorPopupUi>()
            .compile_typst_func::<EffectorPopupUi, EffectorPopupFunc>()
            .render_typst_func::<EffectorPopupFunc>()
            .init_resource::<EffectorPopupFunc>()
            .add_systems(Update, setup_scene);
    }
}

/// Convert screen space scene to world space.
fn setup_scene(mut q_scene: Query<&mut CoordinateSpace, Added<VelystSceneTag<EffectorPopupFunc>>>) {
    if let Ok(mut coordinate_space) = q_scene.get_single_mut() {
        *coordinate_space = CoordinateSpace::WorldSpace;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "effector_popup", layer = 0)]
pub struct EffectorPopupFunc {
    pub message: Option<String>,
    pub button: Option<&'static str>,
    pub button_progress: f64,
}

impl EffectorPopupFunc {
    pub fn clear(&mut self) {
        self.message = None;
        self.button = None;
        self.button_progress = 0.0;
    }

    pub fn has_content(&self) -> bool {
        self.message.is_some() || self.button.is_some()
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/effector_popup.typ"]
pub struct EffectorPopupUi;
