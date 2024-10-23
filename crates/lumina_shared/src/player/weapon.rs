use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use blenvy::*;
use leafwing_input_manager::prelude::*;

use crate::action::PlayerAction;
use crate::SourceEntity;

use super::ammo::{AmmoType, FireAmmo};
use super::spaceship::SpaceShip;
use super::{BlueprintType, PlayerId, PlayerInfoType, PlayerInfos};

pub(super) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (sync_weapon_position, weapon_recharge))
            .add_systems(FixedUpdate, (weapon_direction, weapon_attack).chain());

        app.register_type::<WeaponType>().register_type::<Weapon>();
    }
}

/// Sync [`Weapon`] position to [`SpaceShip`] position.
fn sync_weapon_position(
    q_player: Query<&Transform, (With<SpaceShip>, With<SourceEntity>)>,
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

fn weapon_direction(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapon_transforms: Query<&mut Transform, (With<Weapon>, With<SourceEntity>)>,
    player_infos: Res<PlayerInfos>,
) {
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

                weapon_transform.rotation = Quat::from_rotation_z(direction.to_angle());
            }
        }
    }
}

fn weapon_attack(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_weapons: Query<(&Transform, &Weapon, &mut WeaponStat, &PlayerId), With<SourceEntity>>,
    mut fire_ammo_evw: EventWriter<FireAmmo>,
    player_infos: Res<PlayerInfos>,
) {
    for (action, id) in q_actions.iter() {
        if action.pressed(&PlayerAction::Attack) {
            // Attack!
            if let Some((weapon_transform, weapon, mut weapon_stat, id)) = player_infos
                [PlayerInfoType::Weapon]
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
                    id: *id,
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

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum WeaponType {
    #[default]
    Cannon,
    Missle,
    GattlingGun,
}

impl BlueprintType for WeaponType {
    fn visual_info(&self) -> BlueprintInfo {
        match self {
            WeaponType::Cannon => BlueprintInfo::from_path("levels/WeaponCannonVisual.glb"),
            _ => todo!("{self:?} is not supported yet."),
        }
    }

    fn config_info(&self) -> BlueprintInfo {
        match self {
            WeaponType::Cannon => BlueprintInfo::from_path("levels/WeaponCannonConfig.glb"),
            _ => todo!("{self:?} is not supported yet."),
        }
    }
}

#[derive(Reflect)]
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
