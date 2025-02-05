use bevy::prelude::*;
use lumina_common::prelude::*;

use crate::prelude::SpaceshipType;

pub struct BlueprintColliderPlugin;

impl Plugin for BlueprintColliderPlugin {
    fn build(&self, app: &mut App) {
        app.spawn_blueprint_collider::<SpaceshipType, ()>();
    }
}
