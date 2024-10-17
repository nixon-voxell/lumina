use bevy::prelude::*;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{PlayerAction, ReplicateInputBundle};
use crate::shared::player::{
    LocalPlayer, PlayerId, PlayerInfo, PlayerInfos, SpaceShip, SpaceShipType,
};
use crate::shared::LocalEntity;

use super::ui::Screen;
use super::{LocalClientId, PredictedOrLocal};

mod aim;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(aim::AimPlugin)
            .add_systems(
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
            spaceship_type.model_info(),
            SpawnBlueprint,
            HideUntilReady,
            SpaceShipVisualAdded,
        ));
    }
}

/// Add input for player on player spawn.
fn add_networked_input(
    mut commands: Commands,
    q_predicted: Query<(&PlayerId, Entity), (Added<SpaceShip>, With<Predicted>)>,
    local_client_id: Res<LocalClientId>,
    mut player_infos: ResMut<PlayerInfos>,
) {
    for (player_id, entity) in q_predicted.iter() {
        info!("Spawned player {:?}.", entity);
        let client_id = player_id.0;

        if client_id == local_client_id.0 {
            // Mark our player.
            commands.entity(entity).insert(LocalPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*player_id));
        }

        player_infos.insert(
            client_id,
            PlayerInfo {
                // TODO: Add lobby entity with the correct bits from room id.
                lobby: Entity::PLACEHOLDER,
                player: entity,
                input: None,
            },
        );
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
