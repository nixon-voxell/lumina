use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::{AutoTimer, AutoTimerFinished, AutoTimerPlugin, StartAutoTimerCommandExt};
use crate::utils::ThreadSafe;

/// Plugin for tracking effect and adding cooldown after the effect.
///
/// This works by providing a marker type `T` for the effect and cooldown tracking
/// timer and a configuration `Config` for reading the effect and cooldown duration.
///
/// Users can define multiple marker types that corresponds to multiple configurations.
/// It is the user's responsiblity to ensure the integrity of relation between
/// the configurations and the marker types.
pub struct CooldownEffectPlugin<T, Config>(PhantomData<T>, PhantomData<Config>);

impl<T, Config> Plugin for CooldownEffectPlugin<T, Config>
where
    T: ThreadSafe,
    Config: CooldownEffectConfig,
{
    fn build(&self, app: &mut App) {
        // Prevent duplicate plugins.
        if app.is_plugin_added::<AutoTimerPlugin<Effect<T>, FixedUpdate, CooldownEffectSet>>()
            == false
        {
            app.add_plugins((
                AutoTimerPlugin::<Effect<T>, _, _>::new(FixedUpdate)
                    .with_system_set(CooldownEffectSet),
                AutoTimerPlugin::<Cooldown<T>, _, _>::new(FixedUpdate)
                    .with_system_set(CooldownEffectSet),
            ));
        }

        app.configure_sets(FixedUpdate, CooldownEffectSet)
            .observe(on_effect_finished::<T, Config>);
    }
}

impl<T, Config> Default for CooldownEffectPlugin<T, Config> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Start cooldown timer on effect timer finished.
fn on_effect_finished<T, Config>(
    trigger: Trigger<AutoTimerFinished<Effect<T>>>,
    mut commands: Commands,
    q_configs: Query<&Config>,
) where
    T: ThreadSafe,
    Config: CooldownEffectConfig,
{
    let entity = trigger.entity();
    if let Ok(config) = q_configs.get(entity) {
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.start_auto_timer::<Cooldown<T>>(Duration::from_secs_f32(
                config.cooldown_duration(),
            ));
        }
    }
}

pub type EffectTimer<T> = AutoTimer<Effect<T>>;
pub type CooldownTimer<T> = AutoTimer<Cooldown<T>>;

/// Marker component for a effect timer.
#[derive(Debug, Clone, PartialEq)]
pub struct Effect<T>(PhantomData<T>);

/// Marker component for a cooldown timer.
#[derive(Debug, Clone, PartialEq)]
pub struct Cooldown<T>(PhantomData<T>);

pub trait CooldownEffectConfig: Component {
    fn effect_duration(&self) -> f32;
    fn cooldown_duration(&self) -> f32;
}

pub trait CooldownEffectCommandExt {
    fn start_cooldown_effect<T, Config>(&mut self, entity: Entity)
    where
        T: ThreadSafe,
        Config: CooldownEffectConfig;
}

impl CooldownEffectCommandExt for Commands<'_, '_> {
    fn start_cooldown_effect<T, Config>(&mut self, entity: Entity)
    where
        T: ThreadSafe,
        Config: CooldownEffectConfig,
    {
        self.add(move |world: &mut World| {
            let Some(config) = world.get::<Config>(entity) else {
                return;
            };

            let effect_duration = config.effect_duration();
            if let Some(mut cmd) = world.commands().get_entity(entity) {
                cmd.start_auto_timer::<Effect<T>>(Duration::from_secs_f32(effect_duration));
            }
        });
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CooldownEffectSet;
