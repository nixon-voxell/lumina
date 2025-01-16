use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use crate::utils::propagate_component;

pub(super) struct SourceEntityPlugin;

impl Plugin for SourceEntityPlugin {
    fn build(&self, app: &mut App) {
        // Propagate [`SourceEntity`] component to the children hierarchy.
        app.add_systems(PostUpdate, propagate_component::<SourceEntity>);
    }
}

/// Entity that represents the final source of reference.
/// This is needed to support both local and replicated entities.
///
/// Any children that follows will also have this component added to them.
#[derive(Component, Default, Clone, Copy)]
pub struct SourceEntity;

pub trait SetSourceAppExt {
    fn set_source<C: Component, F: QueryFilter + 'static>(&mut self) -> &mut Self;
}

impl SetSourceAppExt for App {
    fn set_source<C: Component, F: QueryFilter + 'static>(&mut self) -> &mut Self {
        self.add_systems(
            PreUpdate,
            set_source_impl::<C, F>.before(propagate_component::<SourceEntity>),
        )
    }
}

/// Insert [`SourceEntity`] for entities with specific components and filters.
fn set_source_impl<C: Component, F: QueryFilter>(
    mut commands: Commands,
    q_entities: Query<Entity, (With<C>, F, Without<SourceEntity>)>,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(SourceEntity);
        debug!("SOURCE: {entity}.");
    }
}
