use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::server::RoomId;

use crate::utils::{propagate_component, EntityRoomId};

pub(super) struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostProcessCollisions, filter_collisions)
            .add_systems(PostUpdate, propagate_component::<WorldIdx>);
    }
}

fn filter_collisions(mut collisions: ResMut<Collisions>, q_world_id: Query<Option<&WorldIdx>>) {
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

/// Represents the world the [Entity] belongs to.
/// It is used for collision filtering.
/// Id is also interchangable with [RoomId].
#[derive(Component, Deref, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldIdx(pub Option<Entity>);

impl WorldIdx {
    pub fn from_entity(entity: Entity) -> Self {
        Self(Some(entity))
    }

    pub fn room_id(&self) -> RoomId {
        match **self {
            Some(entity) => entity.room_id(),
            None => RoomId(0),
        }
    }
}
