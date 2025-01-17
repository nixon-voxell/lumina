use avian2d::prelude::*;
use bevy::prelude::*;
use blenvy::*;
use lumina_common::prelude::*;

use super::{GameLayer, PlayerId};
use crate::blueprints::LuminaType;

pub struct LuminaPlugin;

impl Plugin for LuminaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLumina>()
            .add_event::<LuminaCollected>()
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

/// Spawns Lumina entities based on trigger events.
fn spawn_lumina(trigger: Trigger<SpawnLumina>, mut commands: Commands) {
    let event = trigger.event();

    let lumina_entity = commands
        .spawn((
            LuminaType::Normal.config_info(),
            SpawnBlueprint,
            SourceEntity,
            Transform::from_xyz(event.position.x, event.position.y, 0.1),
            CollisionLayers::new(GameLayer::Lumina, GameLayer::Spaceship),
            CollidingEntities::default(),
            Sensor,
            RigidBody::Static,
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
    q_luminas: Query<(Entity, &CollidingEntities), With<LuminaStat>>, // Only Luminas.
    mut q_players: Query<(&PlayerId, &mut CollectedLuminas)>, // Only players with CollectedLuminas.
) {
    for (lumina_entity, colliding_entities) in q_luminas.iter() {
        // Filter for players that collided with the Lumina.
        for &player_entity in colliding_entities.iter() {
            if let Ok((player_id, mut collected_luminas)) = q_players.get_mut(player_entity) {
                println!(
                    "Player {:?} collected Lumina {:?}",
                    player_id, lumina_entity
                );

                // Increment the player's pending Lumina count.
                collected_luminas.pending += 1;

                // Despawn the Lumina entity.
                commands.entity(lumina_entity).despawn();

                // Only allow one player to collect the Lumina.
                break;
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

#[derive(Component, Default)]
pub struct CollectedLuminas {
    // Count of Luminas collected by this player.
    pub pending: u32,
}
