use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::game::prelude::*;
use server::*;

use crate::lobby::LobbyRemoval;

pub(super) struct TeleporterPlugin;

impl Plugin for TeleporterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TeleporterInfos>().add_systems(
            Update,
            (setup_teleporter_info, cleanup_teleporter_info).chain(),
        );
    }
}

/// Add teleporter info.
fn setup_teleporter_info(
    q_teleporters: Query<(&TeleporterEnd, &WorldIdx, Entity), Added<WorldIdx>>,
    mut infos: ResMut<TeleporterInfos>,
) {
    for (teleporter, world_id, entity) in q_teleporters.iter() {
        // Find teleporter info using room id.
        if let Some(info) = infos.get_mut(&world_id.room_id()) {
            info.insert(*teleporter, entity);
        } else {
            // Create a new one if not exists.
            infos.insert(
                world_id.room_id(),
                TeleporterInfo([(*teleporter, entity)].into()),
            );
        }
    }
}

/// Remove teleporter info when lobby is removed.
fn cleanup_teleporter_info(
    mut evr_lobby_removal: EventReader<LobbyRemoval>,
    mut infos: ResMut<TeleporterInfos>,
) {
    for lobby_removal in evr_lobby_removal.read() {
        infos.remove(&lobby_removal.0);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TeleporterInfos(HashMap<RoomId, TeleporterInfo>);

#[derive(Deref, DerefMut, Default)]
pub struct TeleporterInfo(HashMap<TeleporterEnd, Entity>);
