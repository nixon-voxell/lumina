use std::marker::PhantomData;

use bevy::prelude::*;
use lightyear::prelude::*;

use crate::prelude::SourceEntity;
use crate::utils::{PlayerId, ThreadSafe};

pub struct CooldownEffectPlugin<T, Config>(PhantomData<T>, PhantomData<Config>);

impl<T, Config> Plugin for CooldownEffectPlugin<T, Config>
where
    T: ThreadSafe,
    Config: CooldownEffectConfig,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (track_effect::<T, Config>, track_cooldown::<T, Config>),
        );
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

pub trait CooldownEffectConfig: Component {
    fn effect_duration(&self) -> f32;
    fn cooldown_duration(&self) -> f32;
}
