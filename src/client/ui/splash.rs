use bevy::prelude::*;
use bevy_motiongfx::{motiongfx_core::UpdateSequenceSet, prelude::*};
use velyst::prelude::*;

use crate::ui::{windowed_func, WindowedFunc};

use super::{state_scoped_scene, Screen};

pub(super) struct SplashUiPlugin;

impl Plugin for SplashUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SplashUi>()
            .compile_typst_func::<SplashUi, SplashFunc>()
            .render_typst_func::<SplashFunc>()
            .init_resource::<SplashFunc>()
            // .add_systems(Startup, setup_animation)
            .add_systems(
                Update,
                (
                    windowed_func::<SplashFunc>,
                    animate_resource::<SplashFunc, f32>.in_set(UpdateSequenceSet),
                )
                    .run_if(in_state(Screen::Splash)),
            );

        state_scoped_scene::<SplashFunc>(app, Screen::Splash);
    }
}

// fn setup_animation(mut commands: Commands, mut func: ResMut<SplashFunc>) {
//     // func.time = time.elapsed_seconds_f64();
//     // commands.spawn(SequencePlayerBundle::from_sequence());
// }

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "splash", layer = 1)]
pub struct SplashFunc {
    width: f64,
    height: f64,
    #[typst_func(named)]
    time: f64,
}

impl WindowedFunc for SplashFunc {
    fn set_width_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/main_menu.typ"]
struct SplashUi;
