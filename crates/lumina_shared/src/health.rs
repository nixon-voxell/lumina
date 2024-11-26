use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The maximum health of the entity.
#[derive(Reflect, Deref, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct MaxHealth(f32);

impl MaxHealth {
    pub fn new(health: f32) -> Self {
        Self(health)
    }
}

impl Component for MaxHealth {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let max_health = **world.entity(entity).get::<Self>().unwrap();

            let mut commands = world.commands();
            commands.entity(entity).insert(Health(max_health));
        });
    }
}

/// The current health of the entity.
#[derive(
    Component, Reflect, Deref, DerefMut, Serialize, Deserialize, Debug, Clone, Copy, PartialEq,
)]
#[reflect(Component)]
pub struct Health(f32);
