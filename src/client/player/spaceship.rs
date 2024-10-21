use bevy::prelude::*;
use blenvy::*;

use crate::shared::action::ReplicateActionBundle;
use crate::shared::player::spaceship::{SpaceShip, SpaceShipType};
use crate::shared::player::{PlayerId, PlayerInfoType, PlayerInfos};
use crate::shared::SourceEntity;

use super::LocalPlayerId;

pub(super) struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_spaceships);
    }
}

// TODO: Zoom out camera during movement/boosting (zoom level depends on velocity?)
// TODO: Add a slight screen shake during boosting.

/// Initialize spaceships into [`SpaceShipInfos`] and add visuals.
fn init_spaceships(
    mut commands: Commands,
    q_spaceships: Query<
        (&SpaceShipType, &PlayerId, Entity),
        (With<SpaceShip>, Added<SourceEntity>),
    >,
    mut player_infos: ResMut<PlayerInfos>,
    local_player_id: Res<LocalPlayerId>,
) {
    for (spaceship_type, id, spaceship_entity) in q_spaceships.iter() {
        commands.entity(spaceship_entity).insert((
            spaceship_type.visual_info(),
            SpawnBlueprint,
            HideUntilReady,
        ));

        player_infos[PlayerInfoType::SpaceShip].insert(*id, spaceship_entity);

        if **local_player_id == *id {
            // Replicate input from client to server.
            let action_entity = commands.spawn(ReplicateActionBundle::new(*id)).id();
            player_infos[PlayerInfoType::Action].insert(*id, action_entity);
        }
    }
}
