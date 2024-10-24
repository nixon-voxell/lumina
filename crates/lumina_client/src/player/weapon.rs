use bevy::prelude::*;
use lumina_shared::player::weapon::WeaponType;
use lumina_shared::prelude::*;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_blueprint_visual::<WeaponType, ()>);
    }
}
