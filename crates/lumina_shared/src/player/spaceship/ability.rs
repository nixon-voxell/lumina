use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::player::PlayerId;

use super::SpaceshipAction;

pub(super) struct SpaceshipAbilityPlugin;

impl Plugin for SpaceshipAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_shadow_ability,
                ability_tracker_systems::<ShadowAbility>(),
                ability_tracker_systems::<HealAbility>(),
            ),
        );
    }
}

fn init_shadow_ability(
    mut commands: Commands,
    q_spaceships: Query<Entity, (Added<SourceEntity>, With<ShadowAbilityConfig>)>,
    q_children: Query<&Children>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    color_materials: Res<Assets<ColorMaterial>>,
) {
    for entity in q_spaceships.iter() {
        let mut origin_colors = OriginColors::default();
        for (color_material, child) in q_children.iter_descendants(entity).filter_map(|e| {
            q_color_materials
                .get(e)
                .ok()
                .and_then(|handle| color_materials.get(handle))
                .map(|color_material| (color_material, e))
        }) {
            origin_colors.push((child, color_material.color));
        }

        commands.entity(entity).insert(origin_colors);
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
                ability.cooldown,
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
        if effect.tick(time.delta()).finished() {
            commands
                .entity(entity)
                .remove::<AbilityEffect>()
                .insert(AbilityCooldown(Timer::from_seconds(
                    config.cooldown,
                    TimerMode::Once,
                )));
        }
    }
}

/// Track ability cooldown timer and remove it after it ends.
fn track_ability_cooldown<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut q_abilities: Query<
        (&mut AbilityCooldown, &PlayerId, Entity),
        (
            Without<AbilityCooldown>,
            With<AbilityConfig<T>>,
            With<SourceEntity>,
        ),
    >,
    time: Res<Time>,
    network_identity: NetworkIdentity,
) {
    for (mut effect, player_id, entity) in q_abilities.iter_mut() {
        if effect.tick(time.delta()).finished()
            // Only server or local player can remove the cooldown for correct syncing.
            && (network_identity.is_server() || player_id.is_local())
        {
            commands.entity(entity).remove::<AbilityCooldown>();
        }
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

/// Cooldown timer based on [`AbilityConfig::cooldown`].
/// While this component is still in effect, [ability action][crate::action::PlayerAction::Ability] cannot be used.
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AbilityCooldown(Timer);

/// Effect timer based on [`AbilityConfig::duration`].
/// While this component is still in effect, [ability action][crate::action::PlayerAction::Ability] cannot be used.
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AbilityEffect(Timer);

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct ShadowAbility {
    /// A color multiplier for the spaceship's material. (Should be a negative value).
    pub strength: f32,
    /// Duration of the ability in seconds.
    pub duration: f32,
}

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct HealAbility {
    /// Radius of the ability.
    pub radius: f32,
    /// Duration of the ability in seconds.
    pub duration: f32,
}

/// Original color of the material.
/// Used for spaceship with [`ShadowAbilityConfig`].
#[derive(Component, Deref, DerefMut, Default, Debug, Clone)]
pub struct OriginColors(Vec<(Entity, Color)>);
