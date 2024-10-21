use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::client::ui::Screen;
use crate::shared::action::PlayerAction;
use crate::shared::player::spawn_blueprint_visual;
use crate::shared::player::weapon::WeaponType;
use crate::shared::SourceEntity;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_blueprint_visual::<WeaponType, ()>)
            .add_systems(OnExit(Screen::Playing), despawn_networked_inputs);
    }
}

// TODO: Do we need this? Is there a more elegant way? Move this to playing.rs?
/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, With<SourceEntity>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}
