use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use server::*;

pub(super) struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLumina>()
            .add_systems(
                Update,
                (
                    setup_objective_area,
                    replicate_setup_ores,
                    replicate_lumina,
                    track_lumina_lifetime,
                    update_used_ores,
                    reset_objective_area,
                ),
            )
            .add_systems(FixedUpdate, lumina_collection)
            .observe(spawn_lumina);
    }
}

/// Setup [ObjectiveArea] with their respective [ObjectiveAreaManager] parent.
fn setup_objective_area(
    q_areas: Query<Entity, Added<ObjectiveArea>>,
    q_parents: Query<&Parent>,
    mut q_managers: Query<&mut ObjectiveAreaManager>,
) {
    for entity in q_areas.iter() {
        for parent in q_parents.iter_ancestors(entity) {
            if let Ok(mut manager) = q_managers.get_mut(parent) {
                manager.areas.push(entity);
                // There should only be one ObjectiveAreaManager parenting an ObjectiveArea.
                break;
            }
        }
    }
}

/// Replicate and setup [OreType] with their respective [ObjectiveArea] parent.
fn replicate_setup_ores(
    mut commands: Commands,
    mut q_ores: Query<(&mut Health, &WorldIdx, Entity), (With<OreType>, Added<WorldIdx>)>,
    q_parents: Query<&Parent>,
    mut q_areas: Query<&mut ObjectiveArea>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (mut health, world_id, entity) in q_ores.iter_mut() {
        for parent in q_parents.iter_ancestors(entity) {
            if let Ok(mut area) = q_areas.get_mut(parent) {
                // Initialize ores as used and have them managed by the ObjectiveAreaManager.
                area.ores.insert_new_used(entity);
                // Used ores have 0.0 health.
                **health.bypass_change_detection() = 0.0;

                // Set area target and replicate.
                commands.entity(entity).insert((
                    ObjectiveAreaTarget(parent),
                    Replicate {
                        sync: SyncTarget {
                            prediction: NetworkTarget::All,
                            interpolation: NetworkTarget::None,
                        },
                        relevance_mode: NetworkRelevanceMode::InterestManagement,
                        ..default()
                    },
                ));

                room_manager.add_entity(entity, world_id.room_id());
                // There should only be one ObjectiveArea parenting an Ore.
                break;
            }
        }
    }
}

/// Move ores with 0.0 health or less into the used pool.
fn update_used_ores(
    q_ores: Query<(&Health, &ObjectiveAreaTarget, Entity), (With<OreType>, Changed<Health>)>,
    mut q_areas: Query<&mut ObjectiveArea>,
) {
    for (health, area, entity) in q_ores.iter() {
        if **health <= 0.0 {
            if let Ok(mut area) = q_areas.get_mut(area.0) {
                area.ores.set_used(entity);
            }
        }
    }
}

/// Replicate [LuminaType] to clients.
fn replicate_lumina(
    mut commands: Commands,
    q_lumina: Query<(&WorldIdx, Entity), (With<LuminaType>, Added<WorldIdx>)>,
    mut room_manager: ResMut<RoomManager>,
) {
    for (world_id, entity) in q_lumina.iter() {
        commands.entity(entity).insert(Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::None,
                interpolation: NetworkTarget::All,
            },
            relevance_mode: NetworkRelevanceMode::InterestManagement,
            ..default()
        });

        room_manager.add_entity(entity, world_id.room_id());
    }
}

/// Handles both collision detection and gameplay effects for Lumina collection.
fn lumina_collection(
    mut commands: Commands,
    q_luminas: Query<(Entity, &CollidingEntities), (With<LuminaStat>, With<SourceEntity>)>,
    mut q_players: Query<(&PlayerId, &mut CollectedLumina)>,
) {
    for (lumina_entity, colliding_entities) in q_luminas.iter() {
        // Filter for players that collided with the Lumina.
        for &player_entity in colliding_entities.iter() {
            if let Ok((player_id, mut collected_luminas)) = q_players.get_mut(player_entity) {
                if **collected_luminas < CollectedLumina::MAX {
                    // Increment the player's pending Lumina count.
                    **collected_luminas += 1;
                    // Despawn the Lumina entity.
                    commands.entity(lumina_entity).despawn();

                    info!(
                        "Player {:?} collected Lumina {:?}",
                        player_id, lumina_entity
                    );

                    // Only allow one player to collect the Lumina.
                    break;
                }
            }
        }
    }
}

/// Reset objective area after timer ends.
fn reset_objective_area(
    mut commands: Commands,
    mut q_areas: Query<(&mut ObjectiveArea, &mut ResetObjectiveArea, Entity)>,
    mut q_ores: Query<(&mut Health, &MaxHealth), With<OreType>>,
    time: Res<Time>,
) {
    for (mut area, mut reset, area_entity) in q_areas.iter_mut() {
        if reset.tick(time.delta()).finished() {
            // Reset all ores' health to max health in this area.
            for ore_entity in area.ores.used().iter() {
                if let Ok((mut health, max_health)) = q_ores.get_mut(*ore_entity) {
                    info!("Replenished health for Ore: {ore_entity}");
                    **health = **max_health;
                }
            }

            // Objective area has been reset, no longer needs to be keep tracked.
            commands.entity(area_entity).remove::<ResetObjectiveArea>();
            area.ores.set_all_unused();
        }
    }
}

/// Tracks Lumina lifetime and despawn expired Lumina.
fn track_lumina_lifetime(
    mut commands: Commands,
    mut q_lumina: Query<(&mut LuminaStat, Entity)>,
    time: Res<Time>,
) {
    for (mut lumina_stat, lumina_entity) in q_lumina.iter_mut() {
        lumina_stat.lifetime -= time.delta_seconds();
        if lumina_stat.lifetime <= 0.0 {
            // Despawn the Lumina entity when its lifetime expires.
            commands.entity(lumina_entity).despawn();
        }
    }
}

/// Spawns Lumina entities based on trigger events.
fn spawn_lumina(trigger: Trigger<SpawnLumina>, mut commands: Commands) {
    let event = trigger.event();

    let lumina_entity = commands
        .spawn((
            LuminaType::Normal.config_info(),
            SpawnBlueprint,
            Transform::from_xyz(event.position.x, event.position.y, 1.0),
        ))
        .id();

    info!(
        "Spawned Lumina entity {:?} at position {:?}",
        lumina_entity, event.position
    );
}

#[derive(Event)]
pub struct SpawnLumina {
    // Position where the Lumina will appear.
    pub position: Position,
}

/// Reset objective area after the timer stops.
#[derive(Component, Deref, DerefMut)]
pub struct ResetObjectiveArea(pub Timer);

#[derive(Component)]
pub struct ObjectiveAreaTarget(Entity);

#[derive(Component, Default, Debug)]
pub struct ObjectiveAreaManager {
    pub areas: Vec<Entity>,
    // pub selected_area: usize,
}
