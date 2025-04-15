use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::action::PlayerAction;
use crate::health::Health;
use crate::player::objective::CollectedLumina;
use crate::player::{GameLayer, PlayerId, PlayerInfoType, PlayerInfos};

pub use ability::*;
pub use movement::*;

mod ability;
mod movement;

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::SpaceshipMovementPlugin,
            ability::SpaceshipAbilityPlugin,
        ))
        .add_systems(FixedUpdate, spaceship_actions)
        .add_systems(PreUpdate, init_spaceships)
        .add_systems(PostUpdate, spaceship_health);
    }
}

/// Initializs spaceships with necessary components.
fn init_spaceships(
    mut commands: Commands,
    q_spaceships: Query<
        Entity,
        (
            With<Spaceship>,
            With<SourceEntity>,
            Or<(Added<Spaceship>, Added<SourceEntity>)>,
        ),
    >,
) {
    for entity in q_spaceships.iter() {
        commands.entity(entity).insert((
            SpaceshipAction::default(),
            SpaceshipMovementBundle::default(),
            CollectedLumina::default(),
            CollisionLayers::new(GameLayer::Spaceship, LayerMask::ALL),
        ));

        debug!("Initialized spaceship: {entity})");
    }
}

/// Map [`PlayerAction`] to [`SpaceshipAction`].
fn spaceship_actions(
    q_actions: Query<(&ActionState<PlayerAction>, &PlayerId), With<SourceEntity>>,
    mut q_spaceships: Query<
        &mut SpaceshipAction,
        (With<Spaceship>, With<SourceEntity>, Without<Dead>),
    >,
    player_infos: Res<PlayerInfos>,
) {
    for (player_action, id) in q_actions.iter() {
        if let Some(mut action) = player_infos[PlayerInfoType::Spaceship]
            .get(id)
            .and_then(|&e| q_spaceships.get_mut(e).ok())
        {
            action.movement_direction = player_action
                // Get direction from action if pressed.
                .pressed(&PlayerAction::Move)
                .then_some(
                    player_action
                        .clamped_axis_pair(&PlayerAction::Move)
                        .and_then(|axis| axis.xy().try_normalize()),
                )
                .flatten();
            action.is_boosting = player_action.pressed(&PlayerAction::Boost);
            action.is_dash = player_action.just_pressed(&PlayerAction::Dash);
            action.is_braking = player_action.pressed(&PlayerAction::Brake);
            action.is_ability = player_action.just_pressed(&PlayerAction::Ability);
        }
    }
}

pub(super) fn spaceship_health(
    mut q_spaceships: Query<
        (&Health, &mut Visibility),
        (Changed<Health>, With<Spaceship>, With<SourceEntity>),
    >,
) {
    for (health, mut viz) in q_spaceships.iter_mut() {
        match **health <= 0.0 {
            true => *viz = Visibility::Hidden,
            false => *viz = Visibility::Inherited,
        }
    }
}

/// Actions on spaceship that is being refreshed every [`FixedUpdate`].
#[derive(Component, Default, Debug, Clone)]
pub struct SpaceshipAction {
    /// Normalized direction of the player's action.
    pub movement_direction: Option<Vec2>,
    /// Is [PlayerAction::Boost] being pressed?
    pub is_boosting: bool,
    /// Is [PlayerAction::Dash] being just pressed?
    pub is_dash: bool,
    /// Is [PlayerAction::Brake] being pressed?
    pub is_braking: bool,
    /// Is [PlayerAction::Ability] being just pressed?
    pub is_ability: bool,
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Spaceship {
    pub movement: MovementConfig,
    pub brake: BrakeConfig,
    pub boost: BoostConfig,
    pub dash: DashConfig,
    pub energy: EnergyConfig,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Dead;
