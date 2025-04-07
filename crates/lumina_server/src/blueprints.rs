use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_client_only,
                replicate_from_server,
                replicate_blueprint,
            )
                .after(GltfBlueprintsSet::AfterSpawn),
        );
    }
}

/// Despawn entities recursively with [`ClientOnly`].
fn despawn_client_only(mut commands: Commands, q_entities: Query<Entity, Added<ClientOnly>>) {
    for entity in q_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Replicate entity from server marked with [`ReplicateFromServer`].
fn replicate_from_server(
    mut commands: Commands,
    mut q_entities: Query<
        (
            &ReplicateFromServer,
            &WorldIdx,
            Has<NoRecursive>,
            Option<&PlayerId>,
            Entity,
        ),
        (Without<SyncTarget>, Without<BlueprintSpawning>),
    >,
    q_children: Query<&Children>,
    q_sync_filter: Query<(), With<HierarchySync>>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (replicate, world_id, no_recursive, player_id, entity) in q_entities.iter_mut() {
        // Will be controlled by player id if it exists.
        let target = player_id
            .map(|&id| NetworkTarget::Single(id.0))
            .unwrap_or_default();

        commands.entity(entity).insert(Replicate {
            sync: SyncTarget {
                prediction: replicate.prediction_target(),
                interpolation: replicate.interpolation_target(),
            },
            controlled_by: ControlledBy {
                target,
                ..default()
            },
            relevance_mode: NetworkRelevanceMode::InterestManagement,
            hierarchy: ReplicateHierarchy {
                recursive: no_recursive,
            },
            ..default()
        });

        if no_recursive == false {
            commands.entity(entity).remove_parent_in_place();
        }

        // Add all child to room for replication to occur correctly.
        for child in q_children
            .iter_descendants(entity)
            // Filter out entities to sync.
            .filter(|&e| q_sync_filter.contains(e))
        {
            room_manager.add_entity(child, world_id.room_id());
        }

        room_manager.add_entity(entity, world_id.room_id());
    }
}

/// Clone [`BlueprintInfo`]'s path to [`ReplicateBlueprint`].
fn replicate_blueprint(
    mut q_blueprints: Query<
        (&mut ReplicateBlueprint, &BlueprintInfo),
        Or<(Changed<BlueprintInfo>, Added<ReplicateBlueprint>)>,
    >,
) {
    for (mut replicate_blueprint, blueprint_info) in q_blueprints.iter_mut() {
        replicate_blueprint.path = blueprint_info.path.clone();
    }
}
