use bevy::prelude::*;
use bevy_transform_interpolation::*;
use lightyear::prelude::*;

pub(super) struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                TransformEasingSet,
                TransformSyncSet,
                TransformSystem::TransformPropagate,
            )
                .chain(),
        );
    }
}

/// Propagate component to the children hierarchy.
pub fn propagate_component<C: Component + Clone>(
    mut commands: Commands,
    q_children: Query<
        (&C, &Children),
        // Just added or the children changes.
        Or<(Added<C>, Changed<Children>)>,
    >,
) {
    for (component, children) in q_children.iter() {
        for entity in children.iter() {
            commands.entity(*entity).insert(component.clone());
        }
    }
}

/// Runs in [`PostUpdate`] after [`TransformEasingSet`] and before [`TransformSystem::TransformPropagate`].
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TransformSyncSet;

pub trait EntityRoomId {
    fn room_id(self) -> server::RoomId;
}

impl EntityRoomId for Entity {
    fn room_id(self) -> server::RoomId {
        server::RoomId(self.to_bits())
    }
}
