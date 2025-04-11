use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use super::Screen;

pub(super) struct SplashUiPlugin;

impl Plugin for SplashUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SplashUi>()
            .compile_typst_func::<SplashUi, SplashFunc>()
            .push_to_main_window::<SplashUi, SplashFunc, _>(
                MainWindowSet::Default,
                in_state(Screen::Splash),
            )
            .init_resource::<SplashFunc>();
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "splash", layer = 1)]
pub struct SplashFunc {
    time: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/splash.typ"]
struct SplashUi;
