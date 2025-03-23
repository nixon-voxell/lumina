use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::action::PlayerAction;

use super::ammo::FireAmmo;
use super::prelude::TeamType;
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
            .add_systems(
                PostUpdate,
                (
                    mimic_spaceship_comp::<Visibility>.after(spaceship_health),
                    mimic_spaceship_comp::<TeamType>,
                ),
            );
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

/// Track the [`WeaponState::magazine()`] and reload when it reaches `0`.
fn weapon_magazine_tracker(
    mut commands: Commands,
    q_weapons: Query<
        (&WeaponState, &Weapon, Entity),
        (
            Changed<WeaponState>,
            Without<WeaponReload>,
            With<SourceEntity>,
        ),
    >,
    q_reload: Query<Entity, With<WeaponReload>>,
) {
    for (state, weapon, entity) in q_weapons.iter() {
        if state.magazine > 0 {
            continue;
        }

        // Check if entity is already reloading
        if q_reload.get(entity).is_ok() {
            continue;
        }

        // If magazine is empty, trigger a full reload
        commands
            .entity(entity)
            .insert(WeaponReload(Timer::from_seconds(
                weapon.reload_duration(),
                TimerMode::Once,
            )));
    }
}

fn weapon_reload(
    mut commands: Commands,
    mut q_weapons: Query<
        (&mut WeaponReload, &mut WeaponState, &Weapon, Entity),
        With<SourceEntity>,
    >,
    time: Res<Time>,
) {
    for (mut reload, mut stat, weapon, entity) in q_weapons.iter_mut() {
        reload.tick(time.delta());

        if reload.finished() {
            stat.magazine = weapon.magazine_size();

            // Remove the reload component since reloading is done
            commands.entity(entity).remove::<WeaponReload>();
        }
    }
}

/// Manually trigger a weapon reload via keybind action by
/// emptying the [`WeaponState::magazine()`] (set to 0).
///
/// This will then be tracked by [`weapon_magazine_tracker()`]
/// and perform the actual reload sequence.
fn weapon_manual_reload(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapons: Query<
        (&mut WeaponState, &Weapon),
        (
            With<SourceEntity>,
            // Do not reload weapons that are reloading.
            Without<WeaponReload>,
        ),
    >,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Reload) {
            if let Some((mut state, weapon)) = player_infos[PlayerInfoType::Weapon]
                .get(id)
                .and_then(|e| q_weapons.get_mut(*e).ok())
            {
                // Do not reload if magazine is full.
                // TODO: Play a one shot sound when this happens?
                if state.magazine() < weapon.magazine_size() {
                    // Trigger a reload by emptying the entire magazine.
                    state.magazine = 0;
                }
            }
        }
    }
}

fn weapon_attack(
    mut commands: Commands,
    q_actions: Query<
        (&ActionState<PlayerAction>, &PlayerId),
        (Without<WeaponReload>, With<SourceEntity>),
    >,
    mut q_weapons: Query<(&Transform, &Weapon, &mut WeaponState, Entity), With<SourceEntity>>,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Attack) == false {
            continue;
        }

        // Attack!
        if let Some((transform, weapon, mut state, weapon_entity)) = player_infos
            [PlayerInfoType::Weapon]
            .get(id)
            .and_then(|e| q_weapons.get_mut(*e).ok())
        {
            if state.can_fire() == false {
                continue;
            }
            state.fire();

            let direction = transform.local_x().xy();
            let position = transform.translation.xy() + direction * weapon.fire_radius;

            // Fire!
            commands.trigger(FireAmmo {
                weapon_entity,
                position,
                direction,
            });
        }
    }
}

fn weapon_recharge(mut q_weapons: Query<&mut WeaponState, With<SourceEntity>>, time: Res<Time>) {
    for mut state in q_weapons.iter_mut() {
        state.recharge.tick(time.delta());
    }
}

/// Mimic component data from spaceship entity to the weapon entity.
fn mimic_spaceship_comp<T: Component + Clone>(
    mut commands: Commands,
    q_spaceships: Query<(&T, &PlayerId), (Changed<T>, With<Spaceship>, With<SourceEntity>)>,
    player_infos: Res<PlayerInfos>,
) {
    for (comp, id) in q_spaceships.iter() {
        if let Some(entity) = player_infos[PlayerInfoType::Weapon].get(id) {
            // Weapons and spaceships might get despawned when changing them
            // in local lobby.
            if let Some(mut cmd) = commands.get_entity(*entity) {
                cmd.insert(comp.clone());
            }
        }
    }
}

// TODO: Implement recoil, add reload.
#[derive(Reflect, Debug)]
#[reflect(Component)]
pub struct Weapon {
    /// Interval in seconds between each fire.
    firing_rate: f32,
    /// Number of bullets the player can fire before the player needs to reload.
    magazine_size: u32,
    /// Recoil force. An impulse force that acts on the opposite of the attack direction.
    recoil: f32,
    /// Radius location where the ammo fires off.
    fire_radius: f32,
    /// Duration in seconds for weapon to reload when [`WeaponState::magazine()`]
    /// is depleted.
    reload_duration: f32,
}

impl Weapon {
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
            let state = WeaponState::new(weapon);
            world.commands().entity(entity).insert(state);
        });
    }
}

/// The state of the current weapon.
#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponState {
    /// Amount of ammo left in the magazine.
    magazine: u32,
    /// Accumulated duration since the last attack from the weapon.
    recharge: Timer,
}

impl WeaponState {
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
pub struct WeaponReload(Timer);
