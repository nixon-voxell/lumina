use bevy::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use super::main_window::push_to_main_window_background;

pub(super) struct EffectorPopupUiPlugin;

impl Plugin for EffectorPopupUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<EffectorPopupUi>()
            .compile_typst_func::<EffectorPopupUi, EffectorPopupFunc>()
            .init_resource::<EffectorPopupFunc>()
            .add_systems(
                Update,
                push_to_main_window_background::<EffectorPopupFunc>().run_if(show_effector_popup),
            );
    }
}

fn show_effector_popup(func: Res<EffectorPopupFunc>) -> bool {
    func.body.is_some()
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "effector_popup", layer = 1)]
pub struct EffectorPopupFunc {
    pub x: f64,
    pub y: f64,
    pub body: Option<Content>,
}

#[derive(TypstPath)]
#[typst_path = "typst/effector_popup.typ"]
struct EffectorPopupUi;
