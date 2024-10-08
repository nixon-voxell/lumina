use bevy::prelude::*;
use bevy_motiongfx::{motiongfx_core::UpdateSequenceSet, prelude::*};
use velyst::prelude::*;

use crate::ui::main_window::push_to_main_window;

use super::Screen;

pub(super) struct SplashUiPlugin;

impl Plugin for SplashUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SplashUi>()
            .compile_typst_func::<SplashUi, SplashFunc>()
            .init_resource::<SplashFunc>()
            // .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    push_to_main_window::<SplashFunc>(),
                    animate_resource::<SplashFunc, f32>.in_set(UpdateSequenceSet),
                )
                    .run_if(in_state(Screen::Splash)),
            );
    }
}

// fn setup_animation(mut commands: Commands, mut func: ResMut<SplashFunc>) {
//     // func.time = time.elapsed_seconds_f64();
//     // commands.spawn(SequencePlayerBundle::from_sequence());
// }

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "splash", layer = 1)]
pub struct SplashFunc {
    time: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/splash.typ"]
struct SplashUi;
