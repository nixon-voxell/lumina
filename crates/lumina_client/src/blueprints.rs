use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::player::ammo::AmmoRef;
use lumina_shared::prelude::*;

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.spawn_blueprint_visual::<AmmoType, Without<AmmoRef>>()
            .spawn_blueprint_visual::<LuminaType, ()>()
            .add_systems(
                PostUpdate,
                (despawn_server_only, spawn_replicated_blueprints).chain(),
            );
    }
}

/// Despawn entities recursively with [`ReplicateFromServer`] or [`ServerOnly`].
fn despawn_server_only(
    mut commands: Commands,
    q_entities: Query<
        Entity,
        (
            Or<(Added<ReplicateFromServer>, Added<ServerOnly>)>,
            Without<Interpolated>,
            Without<Predicted>,
            Without<Confirmed>,
        ),
    >,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawn blueprints replicated from server via [`ReplicateBlueprint`].
fn spawn_replicated_blueprints(
    mut commands: Commands,
    q_replicated_blueprints: Query<
        (&ReplicateBlueprint, Entity),
        (
            Changed<ReplicateBlueprint>,
            Without<BlueprintInfo>,
            // Only spawn blueprints on the predicted/interpolated entities.
            Without<Confirmed>,
            Without<BlueprintReplicated>,
        ),
    >,
) {
    for (replicated_blueprint, entity) in q_replicated_blueprints.iter() {
        if replicated_blueprint.path.is_empty() == false {
            commands.entity(entity).insert((
                replicated_blueprint.info(),
                SpawnBlueprint,
                BlueprintReplicated,
            ));
        }
    }
}

/// Marker component to signify that blueprint has been replicated successfully.
#[derive(Component)]
pub struct BlueprintReplicated;
