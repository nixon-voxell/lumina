use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::action::{PlayerAction, ReplicateActionBundle};
use crate::shared::player::spaceship::{SpaceShip, SpaceShipType};
use crate::shared::player::{PlayerId, PlayerInfo, PlayerInfos};

use super::{LocalClientId, PredictedOrLocal};

pub(super) struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_spaceship_visual,
                add_networked_input.run_if(resource_exists::<LocalClientId>),
            ),
        )
        .add_systems(OnExit(Screen::Playing), despawn_networked_inputs);
    }
}

/// Add visuals for player.
fn add_spaceship_visual(
    mut commands: Commands,
    q_players: Query<
        (&SpaceShipType, Entity),
        (
            PredictedOrLocal,
            With<SpaceShip>,
            // Haven't added visuals yet.
            Without<SpaceShipVisualAdded>,
        ),
    >,
) {
    for (spaceship_type, entity) in q_players.iter() {
        commands.entity(entity).insert((
            spaceship_type.visual_info(),
            SpawnBlueprint,
            HideUntilReady,
            SpaceShipVisualAdded,
        ));
    }
}

/// Add input for player on player spawn.
fn add_networked_input(
    mut commands: Commands,
    q_spaceships: Query<(&PlayerId, Entity), (Added<SpaceShip>, With<Predicted>)>,
    local_client_id: Res<LocalClientId>,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (player_id, spaceship_entity) in q_spaceships.iter() {
        info!("Spawned player {:?}.", spaceship_entity);
        let client_id = player_id.0;

        // TODO: Change to non-local.
        let mut player_info = PlayerInfo::new_local();
        player_info.spaceship = Some(spaceship_entity);

        if client_id == local_client_id.0 {
            // Mark our player.
            commands.entity(spaceship_entity).insert(LocalPlayer);
            // Replicate input from client to server.
            let input = commands.spawn(ReplicateActionBundle::new(*player_id)).id();
            player_info.input = Some(input);
        }

        player_infos.insert(*player_id, player_info);
    }
}

/// Despawn all networked player inputs.
fn despawn_networked_inputs(
    mut commands: Commands,
    // Despawn only networked actions.
    q_actions: Query<Entity, (With<ActionState<PlayerAction>>, Without<LocalEntity>)>,
) {
    for entity in q_actions.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct SpaceShipVisualAdded;
