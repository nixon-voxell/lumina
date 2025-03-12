use std::marker::PhantomData;

use bevy::prelude::*;
use lightyear::prelude::*;

use crate::prelude::SourceEntity;
use crate::utils::{PlayerId, ThreadSafe};

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
        app.add_systems(
            FixedUpdate,
            (track_effect::<T, Config>, track_cooldown::<T, Config>),
        );
    }
}

impl<T, Config> Default for CooldownEffectPlugin<T, Config> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// Track effect timer and remove it + apply cooldown after it ends.
fn track_effect<T: ThreadSafe, Config: CooldownEffectConfig>(
    mut commands: Commands,
    mut q_abilities: Query<
        (&mut Effect<T>, &Config, Entity),
        (Without<Cooldown<T>>, With<SourceEntity>),
    >,
    time: Res<Time>,
) {
    for (mut effect, config, entity) in q_abilities.iter_mut() {
        if effect.finished() {
            commands
                .entity(entity)
                .remove::<Effect<T>>()
                .insert(Cooldown::<T>::new(config.cooldown_duration()));
        }
        effect.tick(time.delta());
    }
}

/// Track cooldown timer and remove it after it ends.
fn track_cooldown<T: ThreadSafe, Config: CooldownEffectConfig>(
    mut commands: Commands,
    mut q_abilities: Query<
        (&mut Cooldown<T>, &PlayerId, Entity),
        (Without<Effect<T>>, With<Config>, With<SourceEntity>),
    >,
    time: Res<Time>,
    network_identity: NetworkIdentity,
) {
    for (mut cooldown, player_id, entity) in q_abilities.iter_mut() {
        if cooldown.finished()
            // Only server or local player can remove the cooldown for correct syncing.
            && (network_identity.is_server() || player_id.is_local())
        {
            commands.entity(entity).remove::<Cooldown<T>>();
        }
        cooldown.tick(time.delta());
    }
}

#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct Cooldown<T>(#[deref] Timer, PhantomData<T>);

impl<T> Cooldown<T> {
    pub fn new(duration_secs: f32) -> Self {
        Self(
            Timer::from_seconds(duration_secs, TimerMode::Once),
            PhantomData,
        )
    }
}

#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct Effect<T>(#[deref] Timer, PhantomData<T>);

impl<T> Effect<T> {
    pub fn new(duration_secs: f32) -> Self {
        Self(
            Timer::from_seconds(duration_secs, TimerMode::Once),
            PhantomData,
        )
    }
}

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
            world
                .commands()
                .entity(entity)
                .insert(Effect::<T>::new(effect_duration));
        });
    }
}
