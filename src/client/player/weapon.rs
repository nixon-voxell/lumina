use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;

use crate::client::ui::Screen;
use crate::shared::action::PlayerAction;
use crate::shared::player::spaceship::SpaceShip;
use crate::shared::player::weapon::{Weapon, WeaponType};
use crate::shared::player::{BlueprintType, PlayerId, PlayerInfoType, PlayerInfos};
use crate::shared::SourceEntity;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_weapons, sync_weapon_position))
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

/// Sync [`Weapon`] position to [`SpaceShip`] position.
fn sync_weapon_position(
    q_player: Query<&Transform, With<SpaceShip>>,
    mut q_weapons: Query<
        (&mut Transform, &PlayerId),
        (Without<SpaceShip>, With<Weapon>, With<SourceEntity>),
    >,
    player_infos: Res<PlayerInfos>,
) {
    for (mut weapon_transform, id) in q_weapons.iter_mut() {
        let Some(spaceship_transform) = player_infos[PlayerInfoType::SpaceShip]
            .get(id)
            .and_then(|e| q_player.get(*e).ok())
        else {
            continue;
        };

        weapon_transform.translation.x = spaceship_transform.translation.x;
        weapon_transform.translation.y = spaceship_transform.translation.y;
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
