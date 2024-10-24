use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::action::ReplicateActionBundle;
use lumina_shared::player::spaceship::{Spaceship, SpaceshipType};
use lumina_shared::prelude::*;

use super::LocalPlayerId;

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_spaceships,
                spawn_networked_action,
                set_source::<Spaceship, With<Predicted>>,
                spawn_blueprint_visual::<SpaceshipType, ()>,
            ),
        );
    }
}

// TODO: Add a slight screen shake during boosting.

/// Initialize spaceships into [`PlayerInfos`].
fn init_spaceships(
    q_spaceships: Query<(&PlayerId, Entity), (With<Spaceship>, Added<SourceEntity>)>,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (id, spaceship_entity) in q_spaceships.iter() {
        player_infos[PlayerInfoType::Spaceship].insert(*id, spaceship_entity);
    }
}

fn spawn_networked_action(
    mut commands: Commands,
    q_spaceships: Query<&PlayerId, (With<Spaceship>, (Added<SourceEntity>, With<Predicted>))>,
    mut player_infos: ResMut<PlayerInfos>,
    local_player_id: Res<LocalPlayerId>,
) {
    for id in q_spaceships.iter() {
        if **local_player_id == *id {
            // Replicate input from client to server.
            let action_entity = commands.spawn(ReplicateActionBundle::new(*id)).id();
            player_infos[PlayerInfoType::Action].insert(*id, action_entity);
        }
    }
}
