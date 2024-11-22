use avian2d::prelude::*;
use bevy::prelude::*;

use crate::utils::propagate_component;

pub(super) struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostProcessCollisions, filter_collisions)
            .add_systems(PostProcessCollisions, propagate_component::<PhysicsWorldId>);
    }
}

fn filter_collisions(
    mut collisions: ResMut<Collisions>,
    q_world_id: Query<Option<&PhysicsWorldId>>,
) {
    collisions.retain(|contacts| {
        match (
            q_world_id.get(contacts.entity1).ok().flatten(),
            q_world_id.get(contacts.entity2).ok().flatten(),
        ) {
            // Ids must match.
            (Some(id0), Some(id1)) => *id0 == *id1,
            // World Id must exists for collision to work.
            _ => false,
        }
    });
}

#[derive(Component, Deref, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PhysicsWorldId(pub u32);
