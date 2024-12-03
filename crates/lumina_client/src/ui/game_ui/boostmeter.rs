use bevy::prelude::*;
use lumina_common::prelude::SourceEntity;
use velyst::prelude::*;

use crate::ui::game_ui::GameUi;

use crate::player::LocalPlayerId;
use lumina_shared::player::spaceship::Boost;
use lumina_shared::player::PlayerId;

pub(super) struct BoostmeterUiPlugin;

impl Plugin for BoostmeterUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameUi>()
            .compile_typst_func::<GameUi, BoostmeterFunc>()
            .init_resource::<BoostmeterFunc>()
            .add_systems(Update, update_boost_meter);
    }
}

/// Update the boost meter UI to reflect energy changes during boosting and regeneration.
fn update_boost_meter(
    q_boosts: Query<(&Boost, &PlayerId), With<SourceEntity>>, // Query for Boost and PlayerId components
    local_player_id: Res<LocalPlayerId>,                      // Local player's ID
    mut boostmeter_func: ResMut<BoostmeterFunc>,              // Boostmeter resource to update
) {
    // Filter boosts by the local player's ID
    for (boost, _player_id) in q_boosts.iter().filter(|(_, id)| **id == local_player_id.0) {
        boostmeter_func.red_height = (boost.energy / boost.max_energy) as f64;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "boostmeter")]
pub struct BoostmeterFunc {
    height: f64,
    width: f64,
    red_height: f64,
}
