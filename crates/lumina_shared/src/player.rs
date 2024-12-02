use std::ops::{Index, IndexMut};

use avian2d::prelude::*;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use spaceship::Boost;
use spaceship::Spaceship;
use strum::EnumCount;
use weapon::Weapon;

use super::action::PlayerAction;

pub mod ammo;
pub mod spaceship;
pub mod spawn_point;
pub mod weapon;

pub mod prelude {
    pub use super::ammo::{Ammo, AmmoDamage, AmmoStat, FireAmmo};
    pub use super::spaceship::Boost;
    pub use super::spaceship::Spaceship;
    pub use super::spawn_point::{SpawnPoint, SpawnPointEntity, SpawnPointUsed};
    pub use super::weapon::{Weapon, WeaponStat};
    pub use super::{PlayerId, PlayerInfoType, PlayerInfos};
}

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spaceship::SpaceshipPlugin,
            weapon::WeaponPlugin,
            ammo::AmmoPlugin,
            spawn_point::SpawnPointPlugin,
        ));

        app.init_resource::<PlayerInfos>()
            .add_systems(PostUpdate, propagate_component::<PlayerId>)
            .add_systems(
                Update,
                (
                    insert_info::<ActionState<PlayerAction>>(PlayerInfoType::Action),
                    insert_info::<Spaceship>(PlayerInfoType::Spaceship),
                    insert_info::<Weapon>(PlayerInfoType::Weapon),
                    insert_info::<Boost>(PlayerInfoType::Boost),
                ),
            );
    }
}

/// Insert entity into [`PlayerInfos`].
fn insert_info<C: Component>(info_type: PlayerInfoType) -> SystemConfigs {
    let system = move |q_entities: Query<(&PlayerId, Entity), (With<C>, Added<SourceEntity>)>,
                       mut player_infos: ResMut<PlayerInfos>| {
        for (id, entity) in q_entities.iter() {
            player_infos[info_type].insert(*id, entity);
        }
    };

    system.into_configs()
}

#[derive(Component, Serialize, Deserialize, Deref, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub ClientId);

impl PlayerId {
    pub const LOCAL: Self = Self(ClientId::Local(u64::MAX));
}

#[derive(EnumCount, Debug, Clone, Copy)]
pub enum PlayerInfoType {
    Action,
    Spaceship,
    Weapon,
    Boost,
}

/// Maps [`PlayerId`] to it's corresponding [`Entity`].
///
/// Note: Number of hashmaps needs to match the number of variants in [`PlayerInfoType`].
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PlayerInfos<const COUNT: usize = { PlayerInfoType::COUNT }>(
    [HashMap<PlayerId, Entity>; COUNT],
);

impl<const COUNT: usize> PlayerInfos<COUNT> {
    pub fn remove_all(&mut self, id: &PlayerId) -> [Option<Entity>; COUNT] {
        let mut removed_entities = [None; COUNT];
        for (i, info) in self.iter_mut().enumerate() {
            removed_entities[i] = info.remove(id);
        }

        removed_entities
    }
}

impl<const COUNT: usize> Index<PlayerInfoType> for PlayerInfos<COUNT> {
    type Output = HashMap<PlayerId, Entity>;

    fn index(&self, index: PlayerInfoType) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<const COUNT: usize> IndexMut<PlayerInfoType> for PlayerInfos<COUNT> {
    fn index_mut(&mut self, index: PlayerInfoType) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<const COUNT: usize> Default for PlayerInfos<COUNT> {
    fn default() -> Self {
        Self(std::array::from_fn(|_| HashMap::default()))
    }
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Spaceship,
    Ammo,
    Wall,
}
