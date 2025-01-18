use bevy::prelude::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

pub(super) struct SourceEntityPlugin;

impl Plugin for SourceEntityPlugin {
    fn build(&self, app: &mut App) {
        app.set_source::<Spaceship, With<Predicted>>()
            .set_source::<Weapon, With<Predicted>>()
            .set_source::<ActionState<PlayerAction>, With<Predicted>>();
    }
}
