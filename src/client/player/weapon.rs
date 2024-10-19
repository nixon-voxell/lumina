use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;

use crate::client::ui::Screen;
use crate::shared::action::PlayerAction;
use crate::shared::player::weapon::{Weapon, WeaponType};
use crate::shared::player::BlueprintType;
use crate::shared::SourceEntity;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_weapons)
            .add_systems(OnExit(Screen::Playing), despawn_networked_inputs);
    }
}

/// Add [`Weapon`] visuals.
fn init_weapons(
    mut commands: Commands,
    q_players: Query<(&WeaponType, Entity), (With<Weapon>, Added<SourceEntity>)>,
) {
    for (weapon_type, entity) in q_players.iter() {
        commands
            .entity(entity)
            .insert((weapon_type.visual_info(), SpawnBlueprint, HideUntilReady));
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
