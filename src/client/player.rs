use bevy::prelude::*;

use super::LocalClientId;

mod aim;
mod spaceship;
mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            aim::AimPlugin,
            spaceship::SpaceShipPlugin,
            weapon::WeaponPlugin,
        ));
    }
}
