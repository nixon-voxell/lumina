use bevy::prelude::*;
use bevy_vello::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

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
    pub body: Option<Content>,
}

#[derive(TypstPath)]
#[typst_path = "typst/effector_popup.typ"]
pub struct EffectorPopupUi;
