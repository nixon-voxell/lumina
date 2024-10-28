use bevy::prelude::*;
use lumina_shared::prelude::*;

use super::{ui::Screen, Connection};

mod aim;
mod spaceship;
mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            aim::AimPlugin,
            spaceship::SpaceshipPlugin,
            weapon::WeaponPlugin,
        ));

        app.init_resource::<LocalPlayerId>()
            .add_systems(OnEnter(Connection::Disconnected), reset_local_player_id)
            .add_systems(OnEnter(Screen::LocalLobby), reset_local_player_id);
    }
}

/// Reset local player id to [`PlayerId::LOCAL`].
fn reset_local_player_id(mut local_player_id: ResMut<LocalPlayerId>) {
    *local_player_id = LocalPlayerId::default();
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct LocalPlayerInfo<'w> {
    pub player_infos: Res<'w, PlayerInfos>,
    pub local_player_id: Res<'w, LocalPlayerId>,
}

impl<'w> LocalPlayerInfo<'w> {
    pub fn get(&self, info_type: PlayerInfoType) -> Option<Entity> {
        self.player_infos[info_type]
            .get(&**self.local_player_id)
            .copied()
    }
}

// Source of truth for retrieving local entities.
#[derive(Resource, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub(super) struct LocalPlayerId(pub PlayerId);

impl Default for LocalPlayerId {
    fn default() -> Self {
        Self(PlayerId::LOCAL)
    }
}
