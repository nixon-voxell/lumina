use avian2d::prelude::*;
use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use super::ui::Screen;
use super::Connection;

mod aim;
mod ammo;
mod name;
mod spaceship;
mod weapon;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            aim::AimPlugin,
            spaceship::SpaceshipPlugin,
            weapon::WeaponPlugin,
            ammo::AmmoPlugin,
            name::NamePlugin,
        ));

        app.init_resource::<LocalPlayerId>()
            .init_resource::<CachedGameStat>()
            .add_systems(OnEnter(Connection::Disconnected), reset_local_player_id)
            .add_systems(
                OnEnter(Screen::LocalLobby),
                (reset_local_player_id, reset_game_stat),
            )
            .add_systems(
                Update,
                (set_physics_world::<RigidBody>, set_physics_world::<Weapon>),
            );
    }
}

fn set_physics_world<T: Component>(
    mut commands: Commands,
    q_entities: Query<Entity, (Added<T>, Without<PhysicsWorldId>)>,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(PhysicsWorldId::default());
    }
}

/// Reset local player id to [`PlayerId::LOCAL`].
fn reset_local_player_id(mut local_player_id: ResMut<LocalPlayerId>) {
    *local_player_id = LocalPlayerId::default();
}

/// Reset local team type to [`None`].
fn reset_game_stat(mut local_team_type: ResMut<CachedGameStat>) {
    *local_team_type = CachedGameStat::default();
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct LocalPlayerInfo<'w> {
    pub player_infos: Res<'w, PlayerInfos>,
    pub local_player_id: Res<'w, LocalPlayerId>,
}

impl LocalPlayerInfo<'_> {
    pub fn get(&self, info_type: PlayerInfoType) -> Option<Entity> {
        self.player_infos[info_type]
            .get(&**self.local_player_id)
            .copied()
    }
}

// Source of truth for retrieving local entities.
#[derive(Resource, Deref, DerefMut, Debug, Clone, Copy, PartialEq)]
pub(super) struct LocalPlayerId(pub PlayerId);

impl Default for LocalPlayerId {
    fn default() -> Self {
        Self(PlayerId::LOCAL)
    }
}

// The local player's team type.
#[derive(Resource, Deref, DerefMut, Default, Debug, Clone, Copy)]
pub(super) struct CachedGameStat(pub GameStat);

#[derive(Default, Debug, Clone, Copy)]
pub(super) struct GameStat {
    pub team_type: Option<TeamType>,
    pub game_score: Option<GameScore>,
}
