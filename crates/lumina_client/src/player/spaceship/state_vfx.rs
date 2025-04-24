use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use bevy_enoki::prelude::*;
use bevy_motiongfx::prelude::ease;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

pub(super) struct StateVfxPlugin;

impl Plugin for StateVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lumina_vfx, smoke_vfx, booster_vfx));
    }
}

/// Animate lumina vfx based on spaceship's [`CollectedLumina`].
fn lumina_vfx(
    q_spaceships: Query<
        (&CollectedLumina, &InPlaceVfxMap),
        (
            Changed<CollectedLumina>,
            With<Spaceship>,
            With<SourceEntity>,
        ),
    >,
    mut q_particles: Query<(&mut ParticleSpawnerState, &mut ParticleEffectInstance)>,
) {
    for (lumina, vfx_map) in q_spaceships.iter() {
        let Some(entities) = vfx_map.get(&InPlaceVfxType::Lumina) else {
            continue;
        };

        let active_state = lumina.0 > 0;
        // Lower value at first, then boost up to maximum.
        let lumina_ratio = ease::quint::ease_in(f32::clamp(
            lumina.0 as f32 / CollectedLumina::MAX as f32,
            0.0,
            1.0,
        ));

        for entity in entities.iter() {
            if let Ok((mut state, mut instance)) = q_particles.get_mut(*entity) {
                state.active = active_state;
                if active_state == false {
                    continue;
                }

                if let Some(instance) = instance.0.as_mut() {
                    instance.spawn_amount = 1.0.lerp(4.0, lumina_ratio) as u32;
                    instance.spawn_rate = 0.1.lerp(0.05, lumina_ratio);
                }
            }
        }
    }
}

/// Animate smoke vfx based on spaceship's [`Health`].
fn smoke_vfx(
    q_spaceships: Query<
        (&Health, &MaxHealth, &InPlaceVfxMap),
        (
            Or<(Changed<Health>, Changed<MaxHealth>)>,
            With<Spaceship>,
            With<SourceEntity>,
        ),
    >,
    mut q_particles: Query<(&mut ParticleSpawnerState, &mut ParticleEffectInstance)>,
) {
    /// Threshold of health before smokes starts animating.
    const THRESHOLD: f32 = 0.8;

    for (health, max_health, vfx_map) in q_spaceships.iter() {
        let Some(entities) = vfx_map.get(&InPlaceVfxType::Smoke) else {
            continue;
        };

        let health_ratio = f32::clamp(**health / **max_health, 0.0, 1.0);
        let active_state = health_ratio < THRESHOLD;

        let smoke_ratio =
            ease::quint::ease_in(f32::inverse_lerp(THRESHOLD, 0.0, health_ratio).max(0.0));

        for entity in entities.iter() {
            if let Ok((mut state, mut instance)) = q_particles.get_mut(*entity) {
                state.active = active_state;
                if active_state == false {
                    continue;
                }

                if let Some(instance) = instance.0.as_mut() {
                    instance.spawn_rate = 0.4.lerp(0.05, smoke_ratio);
                }
            }
        }
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
            Has<ShadowAbilityConfig>,
            Has<AbilityActive>,
            Entity,
        ),
        With<SourceEntity>,
    >,
    mut q_particles: Query<(&mut ParticleSpawnerState, &mut ParticleEffectInstance)>,
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
        has_shadow_ability,
        is_ability_active,
        entity,
    ) in q_spaceships.iter()
    {
        // Ignition.
        let ignition = f32::clamp(**acceleration / movement.linear_acceleration, 0.0, 1.0);
        // Boost.
        let boost_acc = f32::max(0.0, **acceleration - movement.linear_acceleration);
        let boost_acc_size = boost.linear_acceleration;
        let boost_ratio = f32::clamp(boost_acc / boost_acc_size, 0.0, 1.0);

        // TODO: Make a map to the entities.
        for child in q_childrens.iter_descendants(entity) {
            let Ok(mut booster) = q_boosters.get_mut(child) else {
                continue;
            };

            booster.ignition = booster.ignition.lerp(ignition, time.delta_seconds() * 4.0);
            booster.inv_scale = FloatExt::lerp(1.0, 0.6, boost_ratio);

            // Rotation.
            booster.rotation += **rotation_diff;
            booster.rotation = f32::clamp(booster.rotation, -FRAC_PI_4, FRAC_PI_4);
            booster.rotation = booster.rotation.lerp(0.0, time.delta_seconds() * 6.0);
        }

        let Some(vfx_entities) = vfx_map.get(&InPlaceVfxType::BoosterFlakes) else {
            continue;
        };

        let mut active_state = ignition > 0.5;
        // No particle will be spawned when shadow ability is active.
        if has_shadow_ability && is_ability_active {
            active_state = false;
        }

        for vfx_entity in vfx_entities.iter() {
            if let Ok((mut state, mut instance)) = q_particles.get_mut(*vfx_entity) {
                state.active = active_state;

                if let Some(instance) = instance.0.as_mut() {
                    instance.spawn_amount = 3.0.lerp(8.0, boost_ratio) as u32;
                    instance.spawn_rate = 0.06.lerp(0.02, boost_ratio);
                }
            }
        }
    }
}
