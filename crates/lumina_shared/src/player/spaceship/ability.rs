use std::ops::Add;
use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::cooldown_effect::CooldownEffectSet;
use lumina_common::prelude::*;

use crate::health::{Health, MaxHealth};
use crate::player::GameLayer;
use crate::prelude::TeamType;

use super::{AliveQuery, Spaceship, SpaceshipAction};

pub(super) struct SpaceshipAbilityPlugin;

impl Plugin for SpaceshipAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CooldownEffectPlugin::<Ability, ShadowAbilityConfig>::default(),
            CooldownEffectPlugin::<Ability, HealAbilityConfig>::default(),
        ))
        .add_systems(
            FixedUpdate,
            (
                (
                    apply_ability_effect::<ShadowAbility>,
                    apply_ability_effect::<HealAbility>,
                )
                    .after(super::spaceship_actions),
                init_heal_ability,
                (
                    (enable_heal_ability, disable_heal_ability),
                    apply_heal_ability,
                )
                    .chain(),
                cancel_ability.after(CooldownEffectSet),
            ),
        )
        .observe(ability_in)
        .observe(ability_out);
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
            Added<AbilityEffectTimer>,
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
            Added<AbilityCooldownTimer>,
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
        (&ShapeHits, &HealAbilityConfig, &PlayerId, Entity),
        (With<AbilityActive>, With<SourceEntity>),
    >,
    mut q_healths: Query<(&mut Health, &MaxHealth), (With<Spaceship>, With<SourceEntity>)>,
    q_team_types: AliveQuery<&TeamType, (With<Spaceship>, With<SourceEntity>)>,
    time: Res<Time>,
    network_identity: NetworkIdentity,
) {
    let dt = time.delta_seconds();

    for (hits, config, id, entity) in q_spaceships.iter() {
        // Only apply heal ability on local or sever.
        if !(network_identity.is_server() || id.is_local()) {
            continue;
        }

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

/// Apply ability effect on ability action press.
fn apply_ability_effect<T: ThreadSafe>(
    mut commands: Commands,
    q_abilities: AliveQuery<
        (&SpaceshipAction, Entity),
        (
            Without<AbilityCooldownTimer>,
            Without<AbilityEffectTimer>,
            With<SourceEntity>,
            With<AbilityConfig<T>>,
        ),
    >,
) {
    for (action, entity) in q_abilities.iter() {
        if action.is_ability == false {
            continue;
        }

        // Activate ability
        commands.start_cooldown_effect::<Ability, AbilityConfig<T>>(entity);
    }
}

fn ability_in(
    trigger: Trigger<OnAdd, AbilityEffectTimer>,
    mut commands: Commands,
    q_player_ids: Query<&PlayerId>,
    network_identity: NetworkIdentity,
) {
    let entity = trigger.entity();
    if let Ok(id) = q_player_ids.get(entity) {
        if network_identity.is_server() || id.is_local() {
            commands.entity(entity).insert(AbilityActive);
        }
    }
}

fn ability_out(
    trigger: Trigger<OnAdd, AbilityCooldownTimer>,
    mut commands: Commands,
    q_player_ids: Query<&PlayerId>,
    network_identity: NetworkIdentity,
) {
    let entity = trigger.entity();
    if let Ok(id) = q_player_ids.get(entity) {
        if network_identity.is_server() || id.is_local() {
            commands.entity(entity).remove::<AbilityActive>();
        }
    }
}

fn cancel_ability(
    mut commands: Commands,
    mut q_abilities: Query<
        (
            Option<&mut AbilityEffectTimer>,
            Option<&mut AbilityCooldownTimer>,
            Entity,
        ),
        With<CancelAbility>,
    >,
) {
    for (effect, cooldown, entity) in q_abilities.iter_mut() {
        if let Some(mut effect) = effect {
            let duration = effect.duration();
            effect.tick(duration);
        } else if let Some(mut cooldown) = cooldown {
            // Allow some time for vfx to go off.
            let duration = cooldown.duration().saturating_sub(Duration::from_secs(1));
            cooldown.tick(duration);
        } else {
            // Cleanup.
            commands.entity(entity).remove::<CancelAbility>();
        }
    }
}

pub type ShadowAbilityConfig = AbilityConfig<ShadowAbility>;
pub type HealAbilityConfig = AbilityConfig<HealAbility>;

pub type AbilityEffectTimer = EffectTimer<Ability>;
pub type AbilityCooldownTimer = CooldownTimer<Ability>;

/// Marker struct for ability effect cooldown.
#[derive(Clone, PartialEq)]
pub struct Ability;

/// Configuration of an ability.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct AbilityConfig<T> {
    pub duration: f32,
    pub cooldown: f32,
    pub cue_in_duration: f32,
    pub cue_out_duration: f32,
    ability: T,
}

impl<T> AbilityConfig<T> {
    pub fn ability(&self) -> &T {
        &self.ability
    }
}

impl<T: ThreadSafe> CooldownEffectConfig for AbilityConfig<T> {
    fn effect_duration(&self) -> f32 {
        self.duration
    }

    fn cooldown_duration(&self) -> f32 {
        self.cooldown
    }
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShadowAbility {
    /// A color multiplier for the spaceship's material. (Should be a negative value).
    pub strength: f32,
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HealAbility {
    /// Radius of the ability.
    pub radius: f32,
    /// The amount of healing per second.
    pub healing_rate: f32,
}

/// Temporary marker component for canceling ability until a better
/// ability system comes along.
#[derive(Component)]
pub struct CancelAbility;

#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AbilityActive;

// use std::marker::PhantomData;
// use bevy::ecs::system::{IntoObserverSystem, ObserverSystem};

// /// An empty trigger that does nothing.
// fn empty_trigger<E: Event, B: Bundle>(_: Trigger<E, B>) {}

// // fn on_reset(trigger: Trigger<CancelAbility>, mut commands: Commands) {
// //     commands.entity(trigger.entity()).remove::<AbilityEffect>().insert(AbilityCooldown())
// // }

// pub trait AbilitySystem: Bundle + Sized {
//     fn init() -> impl ObserverSystem<OnAdd, Self> {
//         IntoObserverSystem::into_system(empty_trigger)
//     }

//     fn on_enable() -> impl ObserverSystem<EnableAbility, ()> {
//         IntoObserverSystem::into_system(empty_trigger)
//     }

//     fn on_disable() -> impl ObserverSystem<DisableAbility, ()> {
//         IntoObserverSystem::into_system(empty_trigger)
//     }

//     fn on_reset() -> impl ObserverSystem<CancelAbility, ()> {
//         IntoObserverSystem::into_system(empty_trigger)
//     }
// }

// #[derive(Event)]
// pub struct EnableAbility;

// #[derive(Event)]
// pub struct DisableAbility;

// #[derive(Event)]
// pub struct CancelAbility;

// pub struct AbilityEnableTimer();
