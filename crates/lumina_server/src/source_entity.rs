use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::spaceship::Spaceship;
use lumina_shared::player::weapon::Weapon;
use lumina_shared::prelude::*;
use server::*;

pub(super) struct SourceEntityPlugin;

impl Plugin for SourceEntityPlugin {
    fn build(&self, app: &mut App) {
        app.set_source::<Spaceship, With<SyncTarget>>()
            .set_source::<Weapon, With<SyncTarget>>()
            .set_source::<ActionState<PlayerAction>, With<SyncTarget>>();
    }
}