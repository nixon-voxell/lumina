use std::ops::Add;

use avian2d::prelude::*;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::health::{Health, MaxHealth};
use crate::player::{GameLayer, PlayerId};
use crate::prelude::TeamType;

use super::{Spaceship, SpaceshipAction};

pub(super) struct SpaceshipAbilityPlugin;

impl Plugin for SpaceshipAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (
                    ability_tracker_systems::<ShadowAbility>(),
                    ability_tracker_systems::<HealAbility>(),
                )
                    .after(super::spaceship_actions),
                init_heal_ability,
                (
                    (enable_heal_ability, disable_heal_ability),
                    apply_heal_ability,
                )
                    .chain(),
            ),
        );
    }
}

/// Initialize spaceships with [`HealAbilityConfig`] with a [`ShapeCaster`].
fn init_heal_ability(
    mut commands: Commands,
    q_spaceships: Query<(&HealAbilityConfig, Entity), (With<SourceEntity>, Without<ShapeCaster>)>,
) {
    for (config, entity) in q_spaceships.iter() {
        let mut shape_caster = ShapeCaster::new(
            Collider::circle(config.ability().radius),
            Vec2::ZERO,
            0.0,
            Dir2::X,
        )
        // Do not travel anywhere further.
        .with_max_time_of_impact(0.0)
        // Only find spaceship colliders.
        .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Spaceship))
        // Maximum of 3v3 so 6 spaceships will be the max number that can be collided.
        .with_max_hits(6);
        // Disabled by default.
        shape_caster.disable();

        commands.entity(entity).insert(shape_caster);
    }
}

/// Disable shape caster when heal ability is initiated.
fn enable_heal_ability(
    mut q_spaceships: Query<
        &mut ShapeCaster,
        (
            Added<AbilityEffect>,
            With<HealAbilityConfig>,
            With<SourceEntity>,
        ),
    >,
) {
    for mut shape_caster in q_spaceships.iter_mut() {
        shape_caster.enable();
    }
}

/// Disable shape caster when heal ability is completed.
fn disable_heal_ability(
    mut q_spaceships: Query<
        &mut ShapeCaster,
        (
            Added<AbilityCooldown>,
            With<HealAbilityConfig>,
            With<SourceEntity>,
        ),
    >,
) {
    for mut shape_caster in q_spaceships.iter_mut() {
        shape_caster.disable();
    }
}

/// Apply healing effect to spaceships that are inside the radius and also in the same team.
fn apply_heal_ability(
    q_spaceships: Query<
        (&ShapeHits, &HealAbilityConfig, Entity),
        (
            Without<AbilityCooldown>,
            With<AbilityEffect>,
            With<SourceEntity>,
        ),
    >,
    mut q_healths: Query<(&mut Health, &MaxHealth), (With<Spaceship>, With<SourceEntity>)>,
    q_team_types: Query<&TeamType, (With<Spaceship>, With<SourceEntity>)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (hits, config, entity) in q_spaceships.iter() {
        let heal_amount = config.ability().healing_rate * dt;

        if let Ok((mut health, max_health)) = q_healths.get_mut(entity) {
            // Apply effect to ownself.
            **health = health.add(heal_amount).min(**max_health);
        }

        for hit in hits.iter() {
            if let Ok((mut health, max_health)) = q_healths.get_mut(hit.entity) {
                // Apply effect to team mates only.
                if q_team_types.get(entity) == q_team_types.get(hit.entity) {
                    **health = health.add(heal_amount).min(**max_health);
                }
            }
        }
    }
}

fn ability_tracker_systems<T: Send + Sync + 'static>() -> SystemConfigs {
    (
        apply_ability_effect::<T>,
        track_ability_effect::<T>,
        track_ability_cooldown::<T>,
    )
        .into_configs()
}

/// Apply ability effect on ability action press.
fn apply_ability_effect<T: Send + Sync + 'static>(
    mut commands: Commands,
    q_abilities: Query<
        (&SpaceshipAction, &AbilityConfig<T>, Entity),
        (
            Without<AbilityCooldown>,
            Without<AbilityEffect>,
            With<SourceEntity>,
        ),
    >,
) {
    for (action, ability, entity) in q_abilities.iter() {
        if action.is_ability == false {
            continue;
        }

        commands
            .entity(entity)
            .insert(AbilityEffect(Timer::from_seconds(
                ability.duration,
                TimerMode::Once,
            )));
    }
}

/// Track ability effect timer and remove it + apply cooldown after it ends.
fn track_ability_effect<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut q_abilities: Query<
        (&mut AbilityEffect, &AbilityConfig<T>, Entity),
        (Without<AbilityCooldown>, With<SourceEntity>),
    >,
    time: Res<Time>,
) {
    for (mut effect, config, entity) in q_abilities.iter_mut() {
        if effect.finished() {
            commands
                .entity(entity)
                .remove::<AbilityEffect>()
                .insert(AbilityCooldown(Timer::from_seconds(
                    config.cooldown,
                    TimerMode::Once,
                )));
        }
        effect.tick(time.delta());
    }
}

/// Track ability cooldown timer and remove it after it ends.
fn track_ability_cooldown<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut q_abilities: Query<
        (&mut AbilityCooldown, &PlayerId, Entity),
        (
            Without<AbilityEffect>,
            With<AbilityConfig<T>>,
            With<SourceEntity>,
        ),
    >,
    time: Res<Time>,
    network_identity: NetworkIdentity,
) {
    for (mut cooldown, player_id, entity) in q_abilities.iter_mut() {
        if cooldown.finished()
            // Only server or local player can remove the cooldown for correct syncing.
            && (network_identity.is_server() || player_id.is_local())
        {
            commands.entity(entity).remove::<AbilityCooldown>();
        }
        cooldown.tick(time.delta());
    }
}

pub type ShadowAbilityConfig = AbilityConfig<ShadowAbility>;
pub type HealAbilityConfig = AbilityConfig<HealAbility>;

/// Configuration of an ability.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct AbilityConfig<T> {
    duration: f32,
    cooldown: f32,
    ability: T,
}

impl<T> AbilityConfig<T> {
    pub fn ability(&self) -> &T {
        &self.ability
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn cooldown(&self) -> f32 {
        self.cooldown
    }
}

/// Cooldown timer based on [`AbilityConfig::cooldown()`].
/// While this component is still in effect, [ability action][crate::action::PlayerAction::Ability] cannot be used.
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AbilityCooldown(Timer);

/// Effect timer based on [`AbilityConfig::duration()`].
/// While this component is still in effect, [ability action][crate::action::PlayerAction::Ability] cannot be used.
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AbilityEffect(Timer);

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct ShadowAbility {
    /// A color multiplier for the spaceship's material. (Should be a negative value).
    pub strength: f32,
    /// Transition duration of the spaceship's material colors.
    pub transition_duration: f32,
}

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct HealAbility {
    /// Radius of the ability.
    pub radius: f32,
    /// Animation speed of the vfx.
    pub animation_speed: f32,
    /// The amount of healing per second.
    pub healing_rate: f32,
}
