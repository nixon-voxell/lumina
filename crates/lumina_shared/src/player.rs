use std::ops::{Index, IndexMut};

use bevy::ecs::query::QueryFilter;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use spaceship::Spaceship;
use strum::EnumCount;
use weapon::Weapon;

use super::action::PlayerAction;

pub mod ammo;
pub mod spaceship;
pub mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spaceship::SpaceshipPlugin,
            weapon::WeaponPlugin,
            ammo::AmmoPlugin,
        ));

        app.init_resource::<PlayerInfos>()
            .add_systems(PreUpdate, player_id_hierarchy)
            .add_systems(
                Update,
                (
                    insert_info::<ActionState<PlayerAction>>(PlayerInfoType::Action),
                    insert_info::<Spaceship>(PlayerInfoType::Spaceship),
                    insert_info::<Weapon>(PlayerInfoType::Weapon),
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

/// Propagate [`PlayerId`] to the children hierarchy.
fn player_id_hierarchy(
    mut commands: Commands,
    q_children: Query<
        (&Children, &PlayerId),
        // Just added or the children changes.
        Or<(Added<PlayerId>, Changed<Children>)>,
    >,
) {
    for (children, id) in q_children.iter() {
        for entity in children.iter() {
            commands.entity(*entity).insert(*id);
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub ClientId);

impl PlayerId {
    pub const LOCAL: Self = Self(ClientId::Local(u64::MAX));
}

#[derive(EnumCount, Debug, Clone, Copy)]
pub enum PlayerInfoType {
    Action,
    Spaceship,
    Weapon,
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

pub trait BlueprintType: Component {
    fn visual_info(&self) -> BlueprintInfo;
    fn config_info(&self) -> BlueprintInfo;
}

pub fn spawn_blueprint_visual<T: BlueprintType, F: QueryFilter>(
    mut commands: Commands,
    q_blueprints: Query<(&T, Entity), (Added<SourceEntity>, F)>,
) {
    for (blueprint_type, entity) in q_blueprints.iter() {
        commands.entity(entity).insert((
            blueprint_type.visual_info(),
            SpawnBlueprint,
            HideUntilReady,
        ));
    }
}
