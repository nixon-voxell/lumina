use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::client::ui::Screen;
use crate::shared::action::{PlayerAction, ReplicateActionBundle};
use crate::shared::player::weapon::{Weapon, WeaponTarget, WeaponType};
use crate::shared::player::{LocalPlayer, PlayerId, PlayerInfo, PlayerInfos, SpaceShip};
use crate::shared::LocalEntity;

use super::{LocalClientId, PredictedOrLocal};

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_weapon_visual,
                sync_weapon_position,
                add_networked_weapon.run_if(resource_exists::<LocalClientId>),
            ),
        )
        .add_systems(OnExit(Screen::Playing), despawn_networked_inputs);
    }
}

/// Add visuals for weapon.
fn add_weapon_visual(
    mut commands: Commands,
    q_players: Query<
        (&WeaponType, Entity),
        (
            PredictedOrLocal,
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
    mut q_weapons: Query<(&mut Transform, &WeaponTarget), Without<SpaceShip>>,
) {
    for (mut weapon_transform, target) in q_weapons.iter_mut() {
        let Ok(player_transform) = q_player.get(**target) else {
            continue;
        };

        weapon_transform.translation.x = player_transform.translation.x;
        weapon_transform.translation.y = player_transform.translation.y;
    }
}

/// Add input for player on player spawn.
fn add_networked_weapon(
    mut commands: Commands,
    q_weapons: Query<(&PlayerId, Entity), (Added<Weapon>, With<Predicted>)>,
    local_client_id: Res<LocalClientId>,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (player_id, entity) in q_weapons.iter() {
        info!("Spawned player {:?}.", entity);
        let client_id = player_id.0;

        if client_id == local_client_id.0 {
            // Mark our player.
            commands.entity(entity).insert(LocalPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateActionBundle::new(*player_id));
        }

        player_infos.insert(
            client_id,
            PlayerInfo {
                // TODO: Add lobby entity with the correct bits from room id.
                lobby: Entity::PLACEHOLDER,
                spaceship: entity,
                input: None,
            },
        );
    }
}

/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, Without<LocalEntity>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct WeaponVisualAdded;
