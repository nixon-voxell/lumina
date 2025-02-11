use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use bevy_enoki::prelude::*;
use bevy_motiongfx::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::action::ReplicateActionBundle;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

use super::{CachedGameStat, LocalPlayerId};

pub(super) struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InPlaceVfxMapPlugin::<Spaceship>::default())
            .add_systems(
                Update,
                (
                    init_shadow_ability.after(Convert3dTo2dSet),
                    apply_shadow_ability,
                ),
            )
            .add_systems(
                FixedPostUpdate,
                (spawn_networked_action, cache_team_type, booster_vfx),
            );
    }
}

/// Animate booster vfx based on spaceship's acceleration.
fn booster_vfx(
    q_childrens: Query<&Children>,
    q_spaceships: Query<
        (
            &Spaceship,
            &TargetAcceleration,
            &RotationDiff,
            &InPlaceVfxMap,
            Entity,
        ),
        With<SourceEntity>,
    >,
    mut q_states: Query<&mut ParticleSpawnerState>,
    mut q_boosters: Query<&mut BoosterMaterial, With<SourceEntity>>,
    time: Res<Time>,
) {
    for (
        Spaceship {
            movement, boost, ..
        },
        acceleration,
        rotation_diff,
        vfx_map,
        entity,
    ) in q_spaceships.iter()
    {
        // Ignition.
        let ignition = f32::clamp(**acceleration / movement.linear_acceleration, 0.0, 1.0);
        // Boost.
        let boost_acc = f32::max(0.0, **acceleration - movement.linear_acceleration);
        let boost_acc_size = boost.linear_acceleration;

        // TODO: Make a map to the entities.
        for child in q_childrens.iter_descendants(entity) {
            let Ok(mut booster) = q_boosters.get_mut(child) else {
                continue;
            };

            booster.ignition = booster.ignition.lerp(ignition, time.delta_seconds() * 4.0);
            booster.inv_scale = FloatExt::lerp(1.0, 0.6, boost_acc / boost_acc_size);

            // Rotation.
            booster.rotation += **rotation_diff;
            booster.rotation = f32::clamp(booster.rotation, -FRAC_PI_4, FRAC_PI_4);
            booster.rotation = booster.rotation.lerp(0.0, time.delta_seconds() * 6.0);
        }

        if let Some(vfx_entities) = vfx_map.get(&InPlaceVfxType::BoosterFlakes) {
            for vfx_entity in vfx_entities.iter() {
                if let Ok(mut state) = q_states.get_mut(*vfx_entity) {
                    match ignition > 0.5 {
                        true => state.active = true,
                        false => state.active = false,
                    }
                }
            }
        }
    }
}

/// Initialize the original colors of spaceship materials with the [`ShadowAbilityConfig`].
fn init_shadow_ability(
    mut commands: Commands,
    q_spaceships: Query<Entity, (With<SourceEntity>, With<ShadowAbilityConfig>)>,
    q_children: Query<&Children>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    color_materials: Res<Assets<ColorMaterial>>,
    mut blueprint_evr: EventReader<BlueprintEvent>,
) {
    for bp_event in blueprint_evr.read() {
        if let BlueprintEvent::InstanceReady {
            entity,
            blueprint_name,
            ..
        } = bp_event
        {
            if blueprint_name != &SpaceshipType::Assassin.visual_info().name {
                continue;
            }

            // Check if our target entity is a source entity and contains the ability config.
            if q_spaceships.contains(*entity) == false {
                continue;
            }

            // Initialize origin colors of the materials.
            let mut origin_colors = OriginColors::default();
            for (color_material, child) in q_children.iter_descendants(*entity).filter_map(|e| {
                q_color_materials
                    .get(e)
                    .ok()
                    .and_then(|handle| color_materials.get(handle))
                    .map(|color_material| (color_material, e))
            }) {
                origin_colors.push((child, color_material.color));
            }

            commands.entity(*entity).insert(origin_colors);
            info!("Setup origin colors for {entity}");
        }
    }
}

/// Apply shadow ability effect for spaceships.
fn apply_shadow_ability(
    q_spaceships: Query<
        (
            &ShadowAbilityConfig,
            Option<&AbilityEffect>,
            Option<&AbilityCooldown>,
            &OriginColors,
        ),
        (
            With<SourceEntity>,
            Or<(With<AbilityEffect>, With<AbilityCooldown>)>,
        ),
    >,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ability, effect, cooldown, origin_colors) in q_spaceships.iter() {
        for (entity, origin_color) in origin_colors.iter() {
            let Some(color_material) = q_color_materials
                .get(*entity)
                .ok()
                .and_then(|handle| color_materials.get_mut(handle))
            else {
                continue;
            };

            let shadow_ability = ability.ability();
            let strength = shadow_ability.strength;
            let mut transition = 0.0;

            if let Some(effect) = effect {
                transition = ease::cubic::ease_in_out(
                    (effect.elapsed_secs() / shadow_ability.transition_duration).min(1.0),
                );
            } else if let Some(cooldown) = cooldown {
                transition = ease::cubic::ease_in_out(
                    1.0 - (cooldown.elapsed_secs() / shadow_ability.transition_duration).min(1.0),
                );
            }

            color_material.color =
                origin_color.lerp_that(Color::linear_rgb(strength, strength, strength), transition);
        }
    }
}

fn cache_team_type(
    q_spaceships: Query<(&TeamType, &PlayerId), (With<SourceEntity>, Changed<TeamType>)>,
    local_player_id: Res<LocalPlayerId>,
    mut local_team_type: ResMut<CachedGameStat>,
) {
    for (team_type, id) in q_spaceships
        .iter()
        .filter(|(_, &id)| **local_player_id == id)
    {
        local_team_type.team_type = Some(*team_type);
        info!("{id:?} set to team: {team_type:?}");
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

/// Original color of the material.
/// Used for spaceship with [`ShadowAbilityConfig`].
#[derive(Component, Deref, DerefMut, Default, Debug, Clone)]
pub struct OriginColors(Vec<(Entity, Color)>);
