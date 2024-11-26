use bevy::prelude::*;
use velyst::prelude::*;

use crate::ui::game_ui::GameUi;
pub(super) struct BoostmeterUiPlugin;

impl Plugin for BoostmeterUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, BoostmeterFunc>()
            .init_resource::<BoostmeterFunc>()
            .insert_resource(BoostmeterFunc {
                height: 10.0,
                width: 213.0,
                red_height: 0.0,
            })
            .add_systems(Update, update_boost_meter);
    }
}

/// Update the booster fill state
fn update_boost_meter(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut boostmeter_func: ResMut<BoostmeterFunc>,
) {
    // If space is pressed, increase the boost bar's height
    if keys.pressed(KeyCode::Space) {
        boostmeter_func.red_height += 0.5 * (time.delta_seconds() as f64);
        if boostmeter_func.red_height > 1.0 {
            boostmeter_func.red_height = 1.0; // Cap it at 100%
        }
    } else {
        // If space is released, reduce the height
        boostmeter_func.red_height -= 0.5 * (time.delta_seconds() as f64);
        if boostmeter_func.red_height < 0.0 {
            boostmeter_func.red_height = 0.0; // Min is 0%
        }
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "boostmeter")]
pub struct BoostmeterFunc {
    height: f64,
    width: f64,
    red_height: f64,
}
