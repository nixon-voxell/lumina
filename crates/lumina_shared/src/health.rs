use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, init_health);
    }
}

/// Initialized [Health] from [MaxHealth] if not exist.
pub fn init_health(
    mut commands: Commands,
    q_entities: Query<(&MaxHealth, Entity), (Added<MaxHealth>, Without<Health>)>,
) {
    for (max_health, entity) in q_entities.iter() {
        commands.entity(entity).insert(Health(**max_health));
    }
}

/// The maximum health of the entity.
#[derive(Component, Reflect, Deref, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct MaxHealth(f32);

impl MaxHealth {
    pub fn new(health: f32) -> Self {
        Self(health)
    }
}

/// The current health of the entity.
#[derive(
    Component, Reflect, Deref, DerefMut, Serialize, Deserialize, Debug, Clone, Copy, PartialEq,
)]
#[reflect(Component)]
pub struct Health(f32);

impl Health {
    pub fn new(health: f32) -> Self {
        Self(health)
    }
}
