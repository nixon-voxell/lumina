use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use crate::utils::propagate_component;

pub(super) struct SourceEntityPlugin;

impl Plugin for SourceEntityPlugin {
    fn build(&self, app: &mut App) {
        // Propagate [`SourceEntity`] component to the children hierarchy.
        app.add_systems(PreUpdate, propagate_component::<SourceEntity>);
    }
}

/// Insert [`SourceEntity`] for entities with specific components and filters.
pub fn set_source<C: Component, F: QueryFilter>(
    mut commands: Commands,
    q_entities: Query<Entity, (With<C>, F, Without<SourceEntity>)>,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(SourceEntity);
        debug!("SOURCE: {entity}.");
    }
}

/// Entity that represents the final source of reference.
///
/// Any children that follows will also have this component added to them.
#[derive(Component, Default, Clone, Copy)]
pub struct SourceEntity;
