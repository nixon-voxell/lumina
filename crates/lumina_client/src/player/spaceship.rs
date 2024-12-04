use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::action::ReplicateActionBundle;
use lumina_shared::prelude::*;

use super::{CachedGameStat, LocalPlayerId};

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_networked_action, cache_team_type));
    }
}

fn cache_team_type(
    q_spaceships: Query<(&TeamType, &PlayerId), With<SourceEntity>>,
    local_player_id: Res<LocalPlayerId>,
    mut local_team_type: ResMut<CachedGameStat>,
) {
    if local_team_type.team_type.is_some() {
        return;
    }

    for (team_type, _) in q_spaceships
        .iter()
        .filter(|(_, &id)| **local_player_id == id)
    {
        local_team_type.team_type = Some(*team_type);
    }
}

fn spawn_networked_action(
    mut commands: Commands,
    q_spaceships: Query<&PlayerId, (With<Spaceship>, With<Predicted>, Added<SourceEntity>)>,
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
