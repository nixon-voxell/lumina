use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use super::Screen;

pub(super) struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SettingsUi>()
            .compile_typst_func::<SettingsUi, SettingsFunc>()
            .push_to_main_window::<SettingsUi, SettingsFunc, _>(
                MainWindowSet::Foreground,
                |overlay: Res<SettingsOverlay>| overlay.visible, // Render only if visible
            )
            // .recompile_on_interaction::<SettingsFunc>(|func| &mut func.dummy_update)
            .init_resource::<SettingsFunc>()
            .add_systems(
                Update,
                (close_settings).run_if(in_state(Screen::MainMenu)),
            );
    }
}

fn close_settings(
    interactions: InteractionQuery,
    mut overlay: ResMut<SettingsOverlay>) {
    if interactions.pressed("btn:close") {
        overlay.visible = false;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "settings", layer = 1)]
pub struct SettingsFunc;

#[derive(TypstPath)]
#[typst_path = "typst/client/settings.typ"]
pub struct SettingsUi;

#[derive(Resource, Default)]
pub struct SettingsOverlay {
    pub visible: bool,
}
