use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::health::init_health;
use lumina_shared::player::objective::LuminaSpawnArea;
use lumina_shared::player::prelude::*;
use lumina_shared::prelude::*;
use server::*;

use crate::game::PlayerDeath;
use crate::LobbyInfos;

// TODO: Allow for setting this through blender!
pub const OBJECTIVE_AREA_COUNT: usize = 4;

pub(super) struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_objective_area,
                setup_lumina_spawn_area,
                track_lumina_lifetime,
                reset_objective_area,
                lumina_deposition,
            ),
        )
        .add_systems(PostUpdate, setup_ores.before(init_health))
        .add_systems(FixedUpdate, (ore_destruction, lumina_collection))
        .observe(spawn_lumina)
        .observe(drop_lumina_on_death);
    }
}

/// Listen for messages from clients that are triggering a [DepositLumina] event.
fn lumina_deposition(
    mut q_collected_luminas: Query<(&mut CollectedLumina, &TeamType, &PlayerId)>,
    mut q_game_scores: Query<&mut GameScore>,
    mut evr_deposit: EventReader<MessageEvent<DepositLumina>>,
    lobby_infos: Res<LobbyInfos>,
    player_info: Res<PlayerInfos>,
) {
    for deposit in evr_deposit.read() {
        let deposit_client = *deposit.context();

        if let Some((mut collected_lumina, team_type, id)) = player_info[PlayerInfoType::Spaceship]
            .get(&PlayerId(deposit_client))
            .and_then(|&e| q_collected_luminas.get_mut(e).ok())
        {
            info!("{deposit_client:?} triggered a deposit event with {collected_lumina:?}!");
            if let Some(mut game_score) = lobby_infos
                .get(&id.0)
                .and_then(|e| q_game_scores.get_mut(*e).ok())
            {
                match team_type {
                    TeamType::A => {
                        game_score.score =
                            (game_score.score + collected_lumina.0).min(game_score.max_score);
                    }
                    TeamType::B => {
                        game_score.score = game_score.score.saturating_sub(collected_lumina.0);
                    }
                }
            }

            collected_lumina.0 = 0;
        }
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

/// Apply initial stats and setup [OreType] with their respective [ObjectiveArea] parent.
fn setup_ores(
    mut commands: Commands,
    q_entities: Query<Entity, Added<OreType>>,
    q_parents: Query<&Parent>,
    mut q_areas: Query<&mut ObjectiveArea>,
) {
    for entity in q_entities.iter() {
        for parent in q_parents.iter_ancestors(entity) {
            if let Ok(mut area) = q_areas.get_mut(parent) {
                // Initialize ores as used and have them managed by the ObjectiveAreaManager.
                area.ores.insert_new_used(entity);

                commands.entity(entity).insert((
                    ObjectiveAreaTarget(parent),
                    // Ores are spawned with no health.
                    Health::new(0.0),
                    // Which means it's destroyed.
                    OreDestroyed,
                ));
            }
        }
    }
}

/// Setup [LuminaSpawnArea] with their respective [OreType] parent.
fn setup_lumina_spawn_area(
    mut commands: Commands,
    q_spawn_area: Query<Entity, (With<LuminaSpawnArea>, Without<LuminaSpawnAreaInitialized>)>,
    q_ores: Query<(), With<OreType>>,
    q_parents: Query<&Parent>,
) {
    for entity in q_spawn_area.iter() {
        // Find the parent ore.
        for parent in q_parents.iter_ancestors(entity) {
            if q_ores.contains(parent) {
                commands
                    .entity(parent)
                    .insert(LuminaSpawnAreaTarget(entity));
                // Initialized, do not query for this entity again.
                commands.entity(entity).insert(LuminaSpawnAreaInitialized);
                // There should only be one Ore above the LuminaSpawnArea in the hierarchy.
                break;
            }
        }
    }
}

/// Move ores with 0.0 health or less into the used pool and spawn lumina.
fn ore_destruction(
    mut commands: Commands,
    q_ores: Query<
        (
            &Health,
            &ObjectiveAreaTarget,
            &OreType,
            &LuminaSpawnAreaTarget,
            &WorldIdx,
            Entity,
        ),
        (Changed<Health>, Without<OreDestroyed>),
    >,
    mut q_areas: Query<&mut ObjectiveArea>,
    q_spawn_areas: Query<(&GlobalTransform, &LuminaSpawnArea)>,
) {
    for (health, area_target, ore, lumina_spawn_target, &world_id, entity) in q_ores.iter() {
        if **health <= 0.0 {
            commands.entity(entity).insert(OreDestroyed);

            if let Ok(mut area) = q_areas.get_mut(area_target.0) {
                area.ores.set_used(entity);
            }

            if let Ok((transform, spawn_area)) = q_spawn_areas.get(lumina_spawn_target.0) {
                let translation = transform.translation().xy();

                let value = ore.rand_value();
                for _ in 0..value {
                    let radian = rand::random::<f32>() % TAU;
                    let dir = Vec2::from_angle(radian);
                    let distance = rand::random::<f32>() % spawn_area.radius;

                    commands.trigger(SpawnLumina {
                        position: Position(translation + (dir * distance)),
                        world_id,
                    });
                }
            }
        }
    }
}

/// Handles both collision detection and gameplay effects for Lumina collection.
fn lumina_collection(
    mut commands: Commands,
    q_luminas: Query<(&CollidingEntities, Entity), (Changed<CollidingEntities>, With<LuminaStat>)>,
    mut q_players: AliveQuery<(&PlayerId, &mut CollectedLumina)>,
) {
    for (colliding_entities, entity) in q_luminas.iter() {
        // Filter for players that collided with the Lumina.
        for &player_entity in colliding_entities.iter() {
            if let Ok((player_id, mut collected_luminas)) = q_players.get_mut(player_entity) {
                if **collected_luminas < CollectedLumina::MAX {
                    // Increment the player's pending Lumina count.
                    **collected_luminas += 1;
                    info!("Player {:?} collected Lumina {:?}", player_id, entity);

                    // Despawn the Lumina entity.
                    commands.entity(entity).despawn();

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
        if reset.finished() {
            // Reset all ores' health to max health in this area.
            for &ore_entity in area.ores.used().iter() {
                if let Ok((mut health, max_health)) = q_ores.get_mut(ore_entity) {
                    **health = **max_health;
                    commands.entity(ore_entity).remove::<OreDestroyed>();
                    info!("Replenished health for Ore: {ore_entity}");
                }
            }

            // Objective area has been reset, no longer needs to be keep tracked.
            commands.entity(area_entity).remove::<ResetObjectiveArea>();
            area.ores.set_all_unused();
            continue;
        }

        reset.tick(time.delta());
    }
}

/// Tracks Lumina lifetime and despawn expired Lumina.
fn track_lumina_lifetime(
    mut commands: Commands,
    mut q_lumina: Query<(&mut LuminaStat, Entity)>,
    time: Res<Time>,
) {
    for (mut stat, entity) in q_lumina.iter_mut() {
        stat.lifetime -= time.delta_seconds();

        if stat.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns Lumina entities based on trigger events.
fn spawn_lumina(trigger: Trigger<SpawnLumina>, mut commands: Commands) {
    let spawn = trigger.event();

    let lumina_entity = commands
        .spawn((
            LuminaType::Normal.info(),
            SpawnBlueprint,
            Transform::from_xyz(spawn.position.x, spawn.position.y, 1.0),
            spawn.world_id,
        ))
        .id();

    if let Some(sandbox_entity) = spawn.world_id.0 {
        commands.entity(lumina_entity).set_parent(sandbox_entity);
    }

    info!(
        "Spawned Lumina entity {:?} at position {:?}",
        lumina_entity, spawn.position
    );
}

/// Drops Lumina around the player's death position.
fn drop_lumina_on_death(
    trigger: Trigger<PlayerDeath>,
    mut commands: Commands,
    mut q_players: Query<(&mut CollectedLumina, &WorldIdx)>,
) {
    let death = trigger.event();
    if let Ok((mut collected_lumina, world_id)) = q_players.get_mut(trigger.entity()) {
        if collected_lumina.0 > 0 {
            let radius = 2.0 + (collected_lumina.0 as f32 * 0.5);
            for _ in 0..collected_lumina.0 {
                let radian = rand::random::<f32>() % TAU;
                let dir = Vec2::from_angle(radian);
                let distance = rand::random::<f32>() % radius;
                let spawn_position = Position(death.position.0 + (dir * distance));

                commands.trigger(SpawnLumina {
                    position: spawn_position,
                    world_id: *world_id,
                });
            }

            let dropped_count = collected_lumina.0;
            collected_lumina.0 = 0;
            info!(
                "Dropped {} lumina at position {:?} from player death",
                dropped_count, death.position
            );
        }
    }
}

#[derive(Event)]
struct SpawnLumina {
    // Position where the Lumina will appear.
    pub position: Position,
    pub world_id: WorldIdx,
}

/// Marker component when the Ore is being destroyed.
/// Must be removed when it's being replenished.
#[derive(Component)]
pub struct OreDestroyed;

#[derive(Component)]
struct LuminaSpawnAreaTarget(Entity);

/// Marker component for initialized lumina spawn area,
/// specifically when it found its counterpart Ore.
#[derive(Component)]
struct LuminaSpawnAreaInitialized;

/// Reset objective area after the timer stops.
#[derive(Component, Deref, DerefMut)]
pub struct ResetObjectiveArea(pub Timer);

#[derive(Component)]
struct ObjectiveAreaTarget(Entity);

#[derive(Component, Debug)]
pub struct ObjectiveAreaManager {
    pub areas: Vec<Entity>,
    pub selected_index: usize,
}

impl Default for ObjectiveAreaManager {
    fn default() -> Self {
        Self {
            areas: Vec::new(),
            selected_index: rand::random::<u32>() as usize % OBJECTIVE_AREA_COUNT,
        }
    }
}
