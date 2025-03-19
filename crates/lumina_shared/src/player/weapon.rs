use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

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
                (
                    weapon_magazine_tracker,
                    weapon_reload,
                    weapon_recharge,
                    weapon_manual_reload,
                    (weapon_direction, weapon_attack).chain(),
                ),
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

/// Track the [`WeaponStat::magazine()`] and reload when it reaches `0`.
fn weapon_magazine_tracker(
    mut commands: Commands,
    q_weapons: Query<
        (&WeaponStat, &Weapon, Entity),
        (
            Changed<WeaponStat>,
            Without<WeaponReload>,
            With<SourceEntity>,
        ),
    >,
    q_reload: Query<Entity, With<WeaponReload>>,
) {
    for (weapon_stat, weapon, entity) in q_weapons.iter() {
        if weapon_stat.magazine > 0 {
            continue;
        }

        // Check if entity is already reloading
        if q_reload.get(entity).is_ok() {
            continue;
        }

        // If magazine is empty, trigger a full reload
        commands.entity(entity).insert(WeaponReload {
            timer: Timer::from_seconds(weapon.reload_duration(), TimerMode::Once),
            bullets_to_reload: weapon.magazine_size(),
        });
    }
}

fn weapon_reload(
    mut commands: Commands,
    mut q_weapons: Query<(&mut WeaponReload, &mut WeaponStat, &Weapon, Entity), With<SourceEntity>>,
    time: Res<Time>,
) {
    for (mut reload, mut stat, weapon, entity) in q_weapons.iter_mut() {
        reload.timer.tick(time.delta());

        if reload.timer.finished() {
            // Add only the bullets that were missing
            stat.magazine += reload.bullets_to_reload;
            stat.magazine = stat.magazine.min(weapon.magazine_size());

            // Remove the reload component since reloading is done
            commands.entity(entity).remove::<WeaponReload>();
        }
    }
}

fn weapon_manual_reload(
    mut commands: Commands,
    q_actions: Query<
        (&ActionState<PlayerAction>, &PlayerId),
        (With<SourceEntity>, Without<WeaponReload>),
    >,
    q_weapons: Query<(Entity, &WeaponStat, &Weapon), (With<SourceEntity>, Without<WeaponReload>)>,
    q_reload: Query<Entity, With<WeaponReload>>,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Reload) {
            if let Some((entity, weapon_stat, weapon)) = player_infos[PlayerInfoType::Weapon]
                .get(id)
                .and_then(|e| q_weapons.get(*e).ok())
            {
                let missing_bullets = weapon.magazine_size() - weapon_stat.magazine();

                if missing_bullets > 0 {
                    let reload_time_per_bullet =
                        weapon.reload_duration() / weapon.magazine_size() as f32;
                    let total_reload_time = reload_time_per_bullet * missing_bullets as f32;

                    // Check if reload is already in progress
                    if q_reload.get(entity).is_ok() {
                        continue;
                    }

                    commands.entity(entity).insert(WeaponReload {
                        timer: Timer::from_seconds(total_reload_time, TimerMode::Once),
                        bullets_to_reload: missing_bullets,
                    });
                }
            }
        }
    }
}

fn weapon_attack(
    q_actions: Query<
        (&ActionState<PlayerAction>, &PlayerId),
        (Without<WeaponReload>, With<SourceEntity>),
    >,
    mut q_weapons: Query<
        (&Transform, &Weapon, &mut WeaponStat, &PlayerId, &WorldIdx),
        With<SourceEntity>,
    >,
    mut evw_fire_ammo: EventWriter<FireAmmo>,
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
                if weapon_stat.can_fire() == false {
                    continue;
                }
                weapon_stat.fire();

                let direction = weapon_transform.local_x().xy();
                let position = weapon_transform.translation.xy() + direction * weapon.fire_radius;

                // Fire!
                evw_fire_ammo.send(FireAmmo {
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

fn weapon_recharge(mut q_weapons: Query<&mut WeaponStat, With<SourceEntity>>, time: Res<Time>) {
    for mut weapon_stat in q_weapons.iter_mut() {
        weapon_stat.recharge.tick(time.delta());
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
            // Weapons and spaceships might get despawned when changing them
            // in local lobby.
            if let Some(mut cmd) = commands.get_entity(*entity) {
                cmd.insert(*viz);
            }
        }
    }
}

// TODO: Implement recoil, remove/reduce cam shake, add reload.
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
    /// Duration in seconds for weapon to reload when [`WeaponStat::magazine()`]
    /// is depleted.
    reload_duration: f32,
}

impl Weapon {
    pub fn fire_radius(&self) -> f32 {
        self.fire_radius
    }

    pub fn magazine_size(&self) -> u32 {
        self.magazine_size
    }

    pub fn reload_duration(&self) -> f32 {
        self.reload_duration
    }
}

impl Component for Weapon {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let weapon = world.entity(entity).get::<Self>().unwrap();
            let stat = WeaponStat::new(weapon);
            world.commands().entity(entity).insert(stat);
        });
    }
}

/// The stat of the current weapon.
#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponStat {
    /// Amount of ammo left in the magazine.
    magazine: u32,
    /// Accumulated duration since the last attack from the weapon.
    recharge: Timer,
}

impl WeaponStat {
    pub fn new(weapon: &Weapon) -> Self {
        Self {
            magazine: weapon.magazine_size,
            recharge: Timer::from_seconds(weapon.firing_rate, TimerMode::Once),
        }
    }

    pub fn magazine(&self) -> u32 {
        self.magazine
    }

    pub fn recharge(&self) -> &Timer {
        &self.recharge
    }

    pub fn can_fire(&self) -> bool {
        self.magazine != 0 && self.recharge.finished()
    }

    /// Perform a weapon fire action which uses up 1 ammo from
    /// [`Self::magazine()`] and resets [`Self::recharge()`].
    pub fn fire(&mut self) {
        // Use up one ammo.
        self.magazine = self.magazine.saturating_sub(1);
        // Reset the recharge.
        self.recharge.reset();
    }

    pub fn reload(&mut self, weapon: &Weapon) {
        self.magazine = weapon.magazine_size;
    }
}

/// Reload timer based on [`Weapon::reload_duration()`].
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct WeaponReload {
    #[deref]
    pub timer: Timer, // Reload timer
    pub bullets_to_reload: u32, // Number of bullets to reload
}
