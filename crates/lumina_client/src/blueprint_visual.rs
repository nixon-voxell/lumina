use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::{
    ammo::{AmmoRef, AmmoType},
    spaceship::SpaceshipType,
    weapon::WeaponType,
};

pub(super) struct BlueprintVisualPlugin;

impl Plugin for BlueprintVisualPlugin {
    fn build(&self, app: &mut App) {
        app.spawn_blueprint_visual::<SpaceshipType, ()>()
            .spawn_blueprint_visual::<AmmoType, Without<AmmoRef>>()
            .spawn_blueprint_visual::<WeaponType, ()>();
    }
}
