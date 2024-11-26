use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_transform_interpolation::*;

use crate::prelude::SourceEntity;

pub(super) struct PhysicsInterpPlugin;

impl Plugin for PhysicsInterpPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TransformInterpolationPlugin::default());

        app.insert_resource(avian2d::sync::SyncConfig {
            // Physics determines the transform of an entity.
            transform_to_position: false,
            position_to_transform: true,
        })
        .add_systems(PostUpdate, (init_position_sync, init_rotation_sync));
    }
}

/// Insert [`TranslationInterpolation`] for entities with [`Position`] and [`SourceEntity`].
fn init_position_sync(
    mut commands: Commands,
    q_positions: Query<
        Entity,
        (
            With<Position>,
            With<SourceEntity>,
            Without<TranslationInterpolation>,
        ),
    >,
) {
    for entity in q_positions.iter() {
        commands.entity(entity).insert(TranslationInterpolation);
    }
}

/// Insert [`RotationInterpolation`] for entities with [`Rotation`] and [`SourceEntity`].
fn init_rotation_sync(
    mut commands: Commands,
    q_rotations: Query<
        Entity,
        (
            With<Rotation>,
            With<SourceEntity>,
            Without<RotationInterpolation>,
        ),
    >,
) {
    for entity in q_rotations.iter() {
        commands.entity(entity).insert(RotationInterpolation);
    }
}
