use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lumina_common::prelude::*;
use serde::{Deserialize, Serialize};

use crate::action::PlayerAction;
use crate::blueprints::AmmoType;

use super::ammo::FireAmmo;
use super::spaceship::{spaceship_health, Spaceship};
use super::{PlayerId, PlayerInfoType, PlayerInfos};

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_weapon_translation.in_set(TransformSyncSet))
            .add_systems(
                FixedUpdate,
                (weapon_recharge, (weapon_direction, weapon_attack).chain()),
            )
            .add_systems(PostUpdate, weapon_visibility.after(spaceship_health));
    }
}

/// Sync [`Weapon`] translation to [`Spaceship`] translation.
fn sync_weapon_translation(
    q_player: Query<&Transform, (With<Spaceship>, With<SourceEntity>)>,
    mut q_weapons: Query<
        (&mut Transform, &PlayerId),
        (With<Weapon>, With<SourceEntity>, Without<Spaceship>),
    >,
    player_infos: Res<PlayerInfos>,
) {
    for (mut weapon_transform, id) in q_weapons.iter_mut() {
        let Some(spaceship_transform) = player_infos[PlayerInfoType::Spaceship]
            .get(id)
            .and_then(|e| q_player.get(*e).ok())
        else {
            continue;
        };

        let spaceship_translation = spaceship_transform.translation;
        weapon_transform.translation.x = spaceship_translation.x;
        weapon_transform.translation.y = spaceship_translation.y;
    }
}

fn weapon_direction(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapon_transforms: Query<&mut Transform, (With<Weapon>, With<SourceEntity>)>,
    player_infos: Res<PlayerInfos>,
    time: Res<Time>,
) {
    const ROTATION_SPEED: f32 = 20.0;

    for (action, id) in q_actions.iter() {
        if let Some(mut weapon_transform) = player_infos[PlayerInfoType::Weapon]
            .get(id)
            .and_then(|e| q_weapon_transforms.get_mut(*e).ok())
        {
            if action.pressed(&PlayerAction::Aim) == false {
                continue;
            }

            if let Some(direction) = action
                .clamped_axis_pair(&PlayerAction::Aim)
                .map(|axis| axis.xy().normalize_or_zero())
            {
                // Leave the rotation as is if mouse position is exactly at the center.
                if direction == Vec2::ZERO {
                    continue;
                }

                let target_rotation = Quat::from_rotation_z(direction.to_angle());
                weapon_transform.rotation = Quat::slerp(
                    weapon_transform.rotation,
                    target_rotation,
                    time.delta_seconds() * ROTATION_SPEED,
                );
            }
        }
    }
}

fn weapon_attack(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapons: Query<
        (
            &Transform,
            &Weapon,
            &mut WeaponStat,
            &PlayerId,
            &PhysicsWorldId,
        ),
        With<SourceEntity>,
    >,
    mut fire_ammo_evw: EventWriter<FireAmmo>,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Attack) {
            // Attack!
            if let Some((weapon_transform, weapon, mut weapon_stat, &player_id, &world_id)) =
                player_infos[PlayerInfoType::Weapon]
                    .get(id)
                    .and_then(|e| q_weapons.get_mut(*e).ok())
            {
                if weapon_stat.can_fire(weapon.firing_rate) == false {
                    continue;
                }
                weapon_stat.fire();

                let direction = weapon_transform.local_x().xy();
                let position = weapon_transform.translation.xy() + direction * weapon.fire_radius;

                // Fire!
                fire_ammo_evw.send(FireAmmo {
                    player_id,
                    world_id,
                    ammo_type: weapon.ammo_type,
                    position,
                    direction,
                    damage: weapon.damage,
                });
            }
        }
    }
}

fn weapon_recharge(
    mut q_weapons: Query<(&mut WeaponStat, &Weapon), With<SourceEntity>>,
    time: Res<Time>,
) {
    for (mut weapon_stat, weapon) in q_weapons.iter_mut() {
        weapon_stat.recharge = f32::min(
            weapon_stat.recharge + time.delta_seconds(),
            weapon.firing_rate,
        );
    }
}

/// Update weapon visibility based on spaceship visibility.
fn weapon_visibility(
    mut commands: Commands,
    q_spaceships: Query<
        (&Visibility, &PlayerId),
        (Changed<Visibility>, With<Spaceship>, With<SourceEntity>),
    >,
    player_infos: Res<PlayerInfos>,
) {
    for (viz, id) in q_spaceships.iter() {
        if let Some(entity) = player_infos[PlayerInfoType::Weapon].get(id) {
            commands.entity(*entity).insert(*viz);
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Weapon {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
    /// Recoil force. An impulse force that acts on the opposite of the attack direction.
    recoil: f32,
    /// Type of ammo.
    ammo_type: AmmoType,
    /// Damage per ammo hit.
    damage: f32,
    /// Radius location where the ammo fires off.
    fire_radius: f32,
}

impl Weapon {
    pub fn fire_radius(&self) -> f32 {
        self.fire_radius
    }
}

impl Component for Weapon {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let config = world.entity(entity).get::<Self>().unwrap();
            let stat = WeaponStat::from_config(config);
            world.commands().entity(entity).insert(stat);
        });
    }
}

/// The stat of the current weapon.
#[derive(Component)]
pub struct WeaponStat {
    /// Amount of ammo left in the magazine.
    pub magazine: u32,
    /// Accumulated duration since the last attack from the weapon.
    pub recharge: f32,
}

impl WeaponStat {
    pub fn from_config(config: &Weapon) -> Self {
        Self {
            magazine: config.magazine_size,
            recharge: config.firing_rate,
        }
    }

    pub fn can_fire(&self, firing_rate: f32) -> bool {
        self.magazine != 0 && self.recharge >= firing_rate
    }

    /// Perform a weapon fire action which uses up 1 ammo from [`Self::magazine`] and resets [`Self::recharge`].
    pub fn fire(&mut self) {
        // Use up one ammo.
        self.magazine = self.magazine.saturating_sub(1);
        // Reset the recharge.
        self.recharge = 0.0;
    }
}
