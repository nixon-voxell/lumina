use std::ops::{Index, IndexMut};

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use spaceship::SpaceShip;
use strum::EnumCount;
use weapon::Weapon;

use super::action::PlayerAction;
use super::{SetSourceSet, SourceEntity};

pub mod spaceship;
pub mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spaceship::SpaceShipPlugin, weapon::WeaponPlugin));

        app.init_resource::<PlayerInfos>().add_systems(
            PostUpdate,
            (
                insert_info::<ActionState<PlayerAction>>(PlayerInfoType::Action),
                insert_info::<SpaceShip>(PlayerInfoType::SpaceShip),
                insert_info::<Weapon>(PlayerInfoType::Weapon),
            )
                .after(SetSourceSet),
        );
    }
}

fn insert_info<C: Component>(info_type: PlayerInfoType) -> SystemConfigs {
    let system = move |q_entities: Query<(&PlayerId, Entity), (With<C>, Added<SourceEntity>)>,
                       mut player_infos: ResMut<PlayerInfos>| {
        for (id, entity) in q_entities.iter() {
            player_infos[info_type].insert(*id, entity);
        }
    };

    system.into_configs()
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub ClientId);

impl PlayerId {
    pub const LOCAL: Self = Self(ClientId::Local(u64::MAX));
}

#[derive(Default)]
pub struct PlayerRoot;

#[derive(EnumCount, Debug, Clone, Copy)]
pub enum PlayerInfoType {
    Root,
    Action,
    SpaceShip,
    Weapon,
}

impl PlayerInfoType {
    pub fn as_usize(self) -> usize {
        self as usize
    }
}

/// Maps [`PlayerId`] to it's corresponding [`Entity`].
///
/// Note: Number of hashmaps needs to match the number of types in [`PlayerInfoType`].
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

impl<const COUNT: usize> Default for PlayerInfos<COUNT> {
    fn default() -> Self {
        Self(std::array::from_fn(|_| HashMap::default()))
    }
}

impl<const COUNT: usize> Index<PlayerInfoType> for PlayerInfos<COUNT> {
    type Output = HashMap<PlayerId, Entity>;

    fn index(&self, index: PlayerInfoType) -> &Self::Output {
        &self.0[index.as_usize()]
    }
}

impl<const COUNT: usize> IndexMut<PlayerInfoType> for PlayerInfos<COUNT> {
    fn index_mut(&mut self, index: PlayerInfoType) -> &mut Self::Output {
        &mut self.0[index.as_usize()]
    }
}

// TODO: Use this to initialize visuals for all BlueprintTypes
pub trait BlueprintType: Component {
    fn visual_info(&self) -> BlueprintInfo;
    fn config_info(&self) -> BlueprintInfo;
}
