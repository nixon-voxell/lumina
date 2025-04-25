use std::time::Duration;

use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use bevy_enoki::prelude::*;
use blenvy::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

use crate::screens::Screen;
use crate::ui::game_ui::timer::CountdownTimerFunc;

pub(super) struct OreVfxPlugin;

impl Plugin for OreVfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AutoTimerPlugin::<BlinkStart>::default(),
            AutoTimerPlugin::<BlinkEnd>::default(),
        ))
        .add_plugins(OriginColorsPlugin::<(
            Added<BlueprintInstanceReady>,
            With<OreType>,
        )>::default())
        .add_systems(Update, (update_cave_floor, ore_health_dimming))
        .observe(on_blink_start)
        .observe(on_blink_end);
    }
}

fn update_cave_floor(
    mut q_cave_floor: Query<&mut CaveFloorMaterial>,
    timer_func: Res<CountdownTimerFunc>,
    screen: Res<State<Screen>>,
    time: Res<Time>,
) {
    if let Ok(mut cave_floor) = q_cave_floor.get_single_mut() {
        match screen.get() {
            // Sync the cave floor animation time so that it's the same for everyone.
            Screen::InGame => cave_floor.time = timer_func.total_seconds as f32,
            _ => cave_floor.time = time.elapsed_seconds(),
        }
    }
}

/// Dim ore's light based on ore's [`Health`].
fn ore_health_dimming(
    mut commands: Commands,
    q_ores: Query<
        (&OriginColors, &Health, &MaxHealth),
        (Or<(Changed<Health>, Added<OriginColors>)>, With<OreType>),
    >,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    /// Ratio from origin color to complete black for ores with 0 health.
    /// Higher = darker.
    const DIM_RATIO: f32 = 0.6;

    const START_DURATION: f32 = 0.4;
    const START_OFFSET: f32 = 0.1;

    const END_DURATION: f32 = 2.0;
    const END_OFFSET: f32 = 0.5;

    for (origin_colors, health, max_health) in q_ores.iter() {
        let health_ratio = f32::clamp(**health / **max_health, 0.0, 1.0);

        let no_health = (health_ratio - 0.0).abs() < f32::EPSILON;
        let full_health = (health_ratio - 1.0).abs() < f32::EPSILON;

        if no_health || full_health {
            let ratio = if no_health { DIM_RATIO } else { 0.0 };

            for (entity, origin_color) in origin_colors.iter() {
                // Stop the blinking loop.
                commands.entity(*entity).remove::<BlinkColor>();

                // Reset the color.
                let Some(color_material) = q_color_materials
                    .get(*entity)
                    .ok()
                    .and_then(|handle| color_materials.get_mut(handle))
                else {
                    continue;
                };

                color_material.color = origin_color.lerp_that(
                    Color::linear_rgba(0.0, 0.0, 0.0, origin_color.alpha()),
                    ratio,
                );
            }
        } else {
            let inv_health_ratio = 1.0 - health_ratio;
            let shard_count = origin_colors.len();
            // At least one will be affected.
            let affected_count = (shard_count as f32 * inv_health_ratio).round().max(1.0) as usize;

            let target_start_duration = f32::lerp(START_OFFSET, START_DURATION, health_ratio);
            let target_end_duration = f32::lerp(END_OFFSET, END_DURATION, inv_health_ratio);

            for _ in 0..affected_count {
                let shard_index = rand::random_range(0..shard_count);

                let (entity, origin_color) = origin_colors[shard_index];

                commands.entity(entity).insert(BlinkColor {
                    start_color: origin_color,
                    end_color: origin_color.lerp_that(
                        Color::linear_rgba(0.0, 0.0, 0.0, origin_color.alpha()),
                        DIM_RATIO,
                    ),
                    start_duration: target_start_duration
                        + rand::random_range(-START_OFFSET * 0.5..START_OFFSET * 0.5),
                    end_duration: target_end_duration
                        + rand::random_range(-END_OFFSET * 0.5..END_OFFSET * 0.5),
                });
            }
        }
    }
}

fn on_blink_start(
    trigger: Trigger<AutoTimerFinished<BlinkStart>>,
    mut commands: Commands,
    q_blinks: Query<(&BlinkColor, &Handle<ColorMaterial>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();

    if let Ok((blink, material_handle)) = q_blinks.get(entity) {
        // Set the color.
        if let Some(material) = color_materials.get_mut(material_handle) {
            material.color = blink.end_color;
        }

        commands
            .entity(entity)
            .start_auto_timer::<BlinkEnd>(Duration::from_secs_f32(blink.end_duration));
    }
}

fn on_blink_end(
    trigger: Trigger<AutoTimerFinished<BlinkEnd>>,
    mut commands: Commands,
    q_blinks: Query<(&BlinkColor, &Handle<ColorMaterial>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();

    if let Ok((blink, material_handle)) = q_blinks.get(entity) {
        // Set the color.
        if let Some(material) = color_materials.get_mut(material_handle) {
            material.color = blink.start_color;
        }

        commands
            .entity(entity)
            .start_auto_timer::<BlinkStart>(Duration::from_secs_f32(blink.start_duration.max(0.0)));
    }
}

/// Alternates the [`ColorMaterial`] between
/// `start_color` and `end_color`.
#[derive(Default, Debug)]
pub struct BlinkColor {
    /// One of the alternative color during the blinking.
    pub start_color: Color,
    /// One of the alternative color during the blinking.
    pub end_color: Color,
    /// The duration for the start color.
    pub start_duration: f32,
    /// The duration for the end color.
    pub end_duration: f32,
}

impl Component for BlinkColor {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let duration = world.get::<Self>(entity).unwrap().start_duration;

            world
                .commands()
                .entity(entity)
                .start_auto_timer::<BlinkStart>(Duration::from_secs_f32(duration))
                .remove::<AutoTimer<BlinkEnd>>();
        });

        hooks.on_remove(|mut world, entity, _| {
            world
                .commands()
                .entity(entity)
                // Stop all timer.
                .remove::<(AutoTimer<BlinkStart>, AutoTimer<BlinkEnd>)>();
        });
    }
}

/// Marker for the start color duration [`AutoTimer`].
pub struct BlinkStart;

/// Marker for the end color duration [`AutoTimer`].
pub struct BlinkEnd;
