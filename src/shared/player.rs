use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::*;
use spaceship::SpaceShip;
use weapon::Weapon;

use crate::utils::EntityRoomId;

use super::action::PlayerAction;

pub(super) struct PlayerPlugin;

pub mod spaceship;
pub mod weapon;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spaceship::SpaceShipPlugin, weapon::WeaponPlugin));

        app.init_resource::<RootInfos>()
            .init_resource::<ActionInfos>()
            .init_resource::<SpaceShipInfos>()
            .init_resource::<WeaponInfos>();
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub ClientId);

impl PlayerId {
    pub const LOCAL: Self = Self(ClientId::Local(u64::MAX));
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct AllPlayerInfos<'w> {
    pub roots: Res<'w, RootInfos>,
    pub actions: Res<'w, ActionInfos>,
    pub spaceships: Res<'w, SpaceShipInfos>,
    pub weapons: Res<'w, WeaponInfos>,
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct AllPlayerInfosMut<'w> {
    pub roots: ResMut<'w, RootInfos>,
    pub actions: ResMut<'w, ActionInfos>,
    pub spaceships: ResMut<'w, SpaceShipInfos>,
    pub weapons: ResMut<'w, WeaponInfos>,
}

impl AllPlayerInfosMut<'_> {
    pub fn remove_all(&mut self, id: &PlayerId) -> [Option<Entity>; 4] {
        [
            self.roots.remove(id),
            self.actions.remove(id),
            self.spaceships.remove(id),
            self.weapons.remove(id),
        ]
    }
}

pub type RootInfos = PlayerInfos<PlayerRoot>;
pub type ActionInfos = PlayerInfos<PlayerAction>;
pub type SpaceShipInfos = PlayerInfos<SpaceShip>;
pub type WeaponInfos = PlayerInfos<Weapon>;

#[derive(Default)]
pub struct PlayerRoot;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PlayerInfos<T>(#[deref] HashMap<PlayerId, Entity>, PhantomData<T>);

impl<T> Default for PlayerInfos<T> {
    fn default() -> Self {
        Self(HashMap::default(), PhantomData)
    }
}

#[derive(Debug)]
pub struct PlayerInfo {
    /// The lobby entity.
    pub lobby: Entity,
    /// Entity with [`PlayerAction`].
    pub input: Option<Entity>,
    /// Entity with [`crate::shared::player::SpaceShip`].
    pub spaceship: Option<Entity>,
    /// Entity with [`weapon::Weapon`].
    pub weapon: Option<Entity>,
}

impl PlayerInfo {
    pub const LOCAL_LOBBY: Entity = Entity::PLACEHOLDER;

    pub fn new(lobby: Entity) -> Self {
        Self {
            lobby,
            input: None,
            spaceship: None,
            weapon: None,
        }
    }

    pub fn new_local() -> Self {
        Self {
            lobby: Self::LOCAL_LOBBY,
            input: None,
            spaceship: None,
            weapon: None,
        }
    }

    /// Returns the [`server::RoomId`] of the lobby.
    pub fn room_id(&self) -> server::RoomId {
        self.lobby.room_id()
    }
}
