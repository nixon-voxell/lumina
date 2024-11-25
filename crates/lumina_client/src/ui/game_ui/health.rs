use bevy::prelude::*;
use velyst::prelude::*;

use super::main_ui::GameUi;

pub(super) struct HealthUiPlugin;

impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, HealthFunc>()
            .init_resource::<HealthFunc>()
            .insert_resource(HealthFunc {
                current_hp: 100.0, // Set initial HP
                max_hp: 100.0, // Set max HP
            });
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "playerhealth")]
pub struct HealthFunc {
    current_hp: f64,
    max_hp: f64,
}
