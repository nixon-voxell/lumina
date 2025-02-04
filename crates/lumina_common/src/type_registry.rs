use bevy::prelude::*;

use crate::physics::{MassRigidbody, MeshCollider, MeshRigidbody, PrimitiveRigidbody};

pub(super) struct TypeRegistryPlugin;

impl Plugin for TypeRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics
            .register_type::<PrimitiveRigidbody>()
            .register_type::<MeshRigidbody>()
            .register_type::<MeshCollider>()
            .register_type::<MassRigidbody>();
    }
}
