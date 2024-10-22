use bevy::prelude::*;

use crate::shared::player::spawn_blueprint_visual;
use crate::shared::player::weapon::WeaponType;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_blueprint_visual::<WeaponType, ()>);
    }
}
