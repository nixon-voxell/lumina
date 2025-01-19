use avian2d::prelude::*;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::blueprints::LuminaType;
use crate::player::{GameLayer, PlayerId};

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLumina>()
            .add_event::<LuminaCollected>()
            .add_systems(Update, init_lumina)
            .add_systems(
                FixedUpdate,
                ((lumina_collection, track_lumina_lifetime).chain(),),
            )
            .observe(spawn_lumina);
    }
}

/// Event triggered when Lumina is collected.
#[derive(Event)]
pub struct LuminaCollected {
    pub player_id: PlayerId,
    pub position: Vec2,
}

fn init_lumina(mut commands: Commands, q_lumina: Query<Entity, Added<LuminaType>>) {
    for entity in q_lumina.iter() {
        commands.entity(entity).insert(CollisionLayers::new(
            GameLayer::Lumina,
            GameLayer::Spaceship,
        ));
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

/// Handles both collision detection and gameplay effects for Lumina collection.
fn lumina_collection(
    mut commands: Commands,
    q_luminas: Query<(Entity, &CollidingEntities), (With<LuminaStat>, With<SourceEntity>)>,
    mut q_players: Query<(&PlayerId, &mut CollectedLuminas)>,
) {
    for (lumina_entity, colliding_entities) in q_luminas.iter() {
        // Filter for players that collided with the Lumina.
        for &player_entity in colliding_entities.iter() {
            if let Ok((player_id, mut collected_luminas)) = q_players.get_mut(player_entity) {
                if **collected_luminas < CollectedLuminas::MAX {
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

/// Tracks Lumina lifetime despawn expired Lumina.
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LuminaStat {
    pub lifetime: f32,
}

#[derive(Event)]
pub struct SpawnLumina {
    // Position where the Lumina will appear.
    pub position: Position,
    // Duration the Lumina will stay in the world.
    pub lifetime: f32,
}

/// Number of luminas collected by player.
#[derive(
    Component,
    Reflect,
    Serialize,
    Deserialize,
    Deref,
    DerefMut,
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
)]
#[reflect(Component)]
pub struct CollectedLuminas(u8);

impl CollectedLuminas {
    pub const MAX: u8 = 15;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ObjectiveArea {
    /// Area cooldown duration after all luminas have been mined.
    pub cooldown: f32,
    #[reflect(ignore)]
    pub ores: EntityHashSet,
}
