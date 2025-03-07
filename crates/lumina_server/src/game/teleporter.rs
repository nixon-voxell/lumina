use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::game::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::lobby::LobbyRemoval;
use crate::LobbyInfos;

pub(super) struct TeleporterPlugin;

impl Plugin for TeleporterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TeleporterInfos>().add_systems(
            Update,
            (
                (setup_teleporter_info, cleanup_teleporter_info).chain(),
                teleport_player,
            ),
        );
    }
}

fn teleport_player(
    // mut commands: Commands,
    q_global_transforms: Query<&GlobalTransform>,
    mut q_positions: Query<&mut Position>,
    mut evr_teleport: EventReader<MessageEvent<Teleport>>,
    infos: Res<TeleporterInfos>,
    lobby_infos: Res<LobbyInfos>,
    player_infos: Res<PlayerInfos>,
) {
    for teleport in evr_teleport.read() {
        let client_id = teleport.context();

        let Some(spaceship_entity) =
            player_infos[PlayerInfoType::Spaceship].get(&PlayerId(*client_id))
        else {
            continue;
        };

        let Some(&teleporter_entity) = lobby_infos
            .get(client_id)
            .map(|e| e.room_id())
            .and_then(|room_id| infos.get(&room_id))
            .and_then(|info| info.get(&teleport.message().teleporter))
        else {
            continue;
        };

        if let Ok(global_transform) = q_global_transforms.get(teleporter_entity) {
            if let Ok(mut position) = q_positions.get_mut(*spaceship_entity) {
                // Teleport the spaceship.
                let translation = global_transform.translation().xy();
                *position = Position::new(translation);

                // Start the cooldown effect.
                // commands.start_cooldown_effect::<Teleporter, TeleporterStart>(teleporter_entity);
            }
        }
    }
}

/// Add teleporter info.
fn setup_teleporter_info(
    q_teleporters: Query<(&Teleporter, &WorldIdx, Entity), (Added<WorldIdx>, With<TeleporterEnd>)>,
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

/// Maps [`RoomId`] to [`TeleporterInfo`].
#[derive(Resource, Deref, DerefMut, Default)]
pub struct TeleporterInfos(HashMap<RoomId, TeleporterInfo>);

/// Maps [`Teleporter`] ID to [`TeleporterEnd`].
#[derive(Deref, DerefMut, Default)]
pub struct TeleporterInfo(HashMap<Teleporter, Entity>);
