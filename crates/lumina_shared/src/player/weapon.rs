use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::action::PlayerAction;
use crate::player::prelude::*;

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
                    track_weapon_magazine,
                    track_weapon_reload,
                    recharge_weapon,
                    manual_weapon_reload,
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
                .axis_pair(&PlayerAction::Aim)
                .map(|axis| axis.xy().normalize_or_zero())
            {
                // Leave the rotation "as is"
                // if mouse position is exactly at the center.
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

/// Track the [`WeaponMagazine`] and reload when it reaches `0`.
fn track_weapon_magazine(
    mut commands: Commands,
    q_weapons: Query<
        (&WeaponMagazine, &Weapon, Entity),
        (
            Changed<WeaponMagazine>,
            Without<WeaponReload>,
            With<SourceEntity>,
        ),
    >,
    q_reload: Query<Entity, With<WeaponReload>>,
) {
    for (magazine, weapon, entity) in q_weapons.iter() {
        if magazine.0 > 0 {
            continue;
        }

        // Check if entity is already reloading
        if q_reload.get(entity).is_ok() {
            continue;
        }

        // Calculate chunks needed to fill the magazine from 0
        let bullets_needed = weapon.magazine_size();
        let chunks_needed =
            (bullets_needed as f32 / weapon.reload_chunk_size() as f32).ceil() as u32;

        // Magazine is already 0, start reload
        commands.entity(entity).insert(WeaponReload::new(
            weapon.reload_chunk_size,
            weapon.reload_duration,
            chunks_needed,
        ));
    }
}

fn track_weapon_reload(
    mut commands: Commands,
    mut q_weapons: Query<
        (&mut WeaponReload, &mut WeaponMagazine, &Weapon, Entity),
        With<SourceEntity>,
    >,
    time: Res<Time>,
) {
    for (mut reload, mut magazine, weapon, entity) in q_weapons.iter_mut() {
        reload.timer.tick(time.delta());

        if reload.timer.finished() {
            // Calculate which chunk is currently filling
            let chunk_size = reload.chunk_size;
            let current_full_chunks = magazine.0 / chunk_size;

            // Fill the next chunk completely
            let next_chunk_idx = current_full_chunks + 1;
            let next_chunk_bullets = next_chunk_idx * chunk_size;

            // Make sure magazine capacity not exceeding
            magazine.0 = next_chunk_bullets.min(weapon.magazine_size());

            reload.current_chunk += 1;

            // Check if all chunks are reloaded or magazine is full
            if reload.current_chunk >= reload.total_chunks || magazine.0 >= weapon.magazine_size() {
                commands.entity(entity).remove::<WeaponReload>();
            } else {
                // Reset the timer for the next chunk
                reload.timer.reset();
            }
        }
    }
}

/// Manually trigger a weapon reload via keybind action by
/// emptying the [`WeaponMagazine`] (set to 0).
///
/// This will then be tracked by [`track_weapon_magazine()`]
/// and perform the actual reload sequence.
fn manual_weapon_reload(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapons: Query<
        (&mut WeaponMagazine, &Weapon, Entity),
        (With<SourceEntity>, Without<WeaponReload>),
    >,
    mut commands: Commands,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Reload) {
            if let Some((magazine, weapon, entity)) = player_infos[PlayerInfoType::Weapon]
                .get(id)
                .and_then(|e| q_weapons.get_mut(*e).ok())
            {
                // Do not reload if magazine is full.
                if magazine.0 >= weapon.magazine_size() {
                    continue;
                }

                // Calculate how many chunks are not full (including partially filled chunks)
                let chunk_size = weapon.reload_chunk_size();
                let total_magazine_chunks = (weapon.magazine_size() + chunk_size - 1) / chunk_size;
                let full_chunks = magazine.0 / chunk_size;
                let chunks_to_reload = total_magazine_chunks - full_chunks;

                // Start reload
                commands.entity(entity).insert(WeaponReload::new(
                    chunk_size,
                    weapon.reload_duration,
                    chunks_to_reload,
                ));
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
    mut q_weapons: Query<
        (
            &Transform,
            &Weapon,
            &mut WeaponMagazine,
            &mut WeaponRecharge,
            Entity,
        ),
        With<SourceEntity>,
    >,
    q_dead_spaceships: DeadQuery<(), (With<Spaceship>, With<SourceEntity>)>,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Attack) == false {
            continue;
        }

        // Validate spaceship state
        let Some(spaceship_entity) = player_infos[PlayerInfoType::Spaceship].get(id) else {
            debug!(
                "Weapon attack rejected: No spaceship for player_id {:?}",
                id
            );
            continue;
        };

        if q_dead_spaceships.contains(*spaceship_entity) {
            debug!(
                "Weapon attack rejected: Spaceship {:?} is dead or has zero health (player_id: {:?})",
                spaceship_entity, id
            );
            continue;
        }

        // Attack!
        if let Some((transform, weapon, mut magazine, mut recharge, weapon_entity)) = player_infos
            [PlayerInfoType::Weapon]
            .get(id)
            .and_then(|e| q_weapons.get_mut(*e).ok())
        {
            if magazine.0 == 0 || recharge.finished() == false {
                continue;
            }

            // Use up one ammo.
            magazine.0 = magazine.saturating_sub(1);
            // Reset the recharge.
            recharge.reset();

            // Cancel any ongoing reload.
            commands.entity(weapon_entity).remove::<WeaponReload>();

            let direction = transform.local_x().xy();
            let position = transform.translation.xy() + direction * weapon.fire_radius;

            // Fire!
            commands.trigger(FireAmmo {
                weapon_entity,
                position,
                direction,
            });
            debug!("Weapon fired for player_id {:?}", id);
        }
    }
}

fn recharge_weapon(mut q_weapons: Query<&mut WeaponRecharge, With<SourceEntity>>, time: Res<Time>) {
    for mut recharge in q_weapons.iter_mut() {
        recharge.tick(time.delta());
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
                cmd.try_insert(comp.clone());
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
    /// Duration in seconds for weapon to reload when
    /// [`WeaponMagazine`] is depleted.
    reload_duration: f32,
    /// Number of bullets reloaded per chunk.
    reload_chunk_size: u32,
}

impl Weapon {
    pub fn magazine_size(&self) -> u32 {
        self.magazine_size
    }

    pub fn reload_duration(&self) -> f32 {
        self.reload_duration
    }

    pub fn reload_chunk_size(&self) -> u32 {
        self.reload_chunk_size
    }
}

impl Component for Weapon {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let weapon = world.entity(entity).get::<Self>().unwrap();
            let bundle = (WeaponMagazine::new(weapon), WeaponRecharge::new(weapon));

            world.commands().entity(entity).insert(bundle);
        });
    }
}

#[derive(Component, Deref, DerefMut, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponMagazine(pub u32);

impl WeaponMagazine {
    pub fn new(weapon: &Weapon) -> Self {
        Self(weapon.magazine_size)
    }
}

#[derive(Component, Deref, DerefMut, Debug, Clone)]
pub struct WeaponRecharge(pub Timer);

impl WeaponRecharge {
    pub fn new(weapon: &Weapon) -> Self {
        Self(Timer::from_seconds(weapon.firing_rate, TimerMode::Once))
    }
}

/// Reload timer for a chunk-based reload, handling multiple chunks.
#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponReload {
    pub timer: Timer,
    pub chunk_size: u32,
    pub total_chunks: u32,
    pub current_chunk: u32,
}

impl WeaponReload {
    pub fn new(chunk_size: u32, reload_duration: f32, total_chunks: u32) -> Self {
        // Timer is per chunk
        Self {
            timer: Timer::from_seconds(reload_duration, TimerMode::Once),
            chunk_size,
            total_chunks,
            current_chunk: 0,
        }
    }
}
