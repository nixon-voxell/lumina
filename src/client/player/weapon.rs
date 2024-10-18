use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;

use crate::client::ui::Screen;
use crate::shared::action::PlayerAction;
use crate::shared::player::spaceship::SpaceShip;
use crate::shared::player::weapon::{Weapon, WeaponType};
use crate::shared::player::{PlayerId, SpaceShipInfos};
use crate::shared::SourceEntity;

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_weapon_visual, sync_weapon_position))
            .add_systems(OnExit(Screen::Playing), despawn_networked_inputs);
    }
}

/// Add visuals for weapon.
fn add_weapon_visual(
    mut commands: Commands,
    q_players: Query<
        (&WeaponType, Entity),
        (
            With<SourceEntity>,
            With<Weapon>,
            // Haven't added visuals yet.
            Without<WeaponVisualAdded>,
        ),
    >,
) {
    for (weapon_type, entity) in q_players.iter() {
        commands.entity(entity).insert((
            weapon_type.visual_info(),
            SpawnBlueprint,
            HideUntilReady,
            WeaponVisualAdded,
        ));
    }
}

fn sync_weapon_position(
    q_player: Query<&Transform, With<SpaceShip>>,
    mut q_weapons: Query<(&mut Transform, &PlayerId), Without<SpaceShip>>,
    spaceship_infos: Res<SpaceShipInfos>,
) {
    for (mut weapon_transform, id) in q_weapons.iter_mut() {
        let Some(player_transform) = spaceship_infos.get(id).and_then(|e| q_player.get(*e).ok())
        else {
            continue;
        };

        weapon_transform.translation.x = player_transform.translation.x;
        weapon_transform.translation.y = player_transform.translation.y;
    }
}

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

#[derive(Component)]
struct WeaponVisualAdded;
