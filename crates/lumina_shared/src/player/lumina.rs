use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use blenvy::*;
use lumina_common::prelude::*;

use super::{GameLayer, PlayerId};
use crate::blueprints::LuminaType;

pub struct LuminaPlugin;

impl Plugin for LuminaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollectedLuminas>()
            .add_event::<SpawnLumina>()
            .add_event::<LuminaCollected>()
            .add_systems(
                FixedUpdate,
                (
                    spawn_lumina,
                    (lumina_collection, track_lumina_lifetime).chain(),
                ),
            );
    }
}

/// Event triggered when Lumina is collected.
#[derive(Event)]
pub struct LuminaCollected {
    pub player_id: PlayerId,
    pub position: Vec2,
}

// TODO: Needs to turn to actual Lumina spawning
/// Spawns Lumina entities based on events.
fn spawn_lumina(mut commands: Commands, mut spawn_lumina_evr: EventReader<SpawnLumina>) {
    for event in spawn_lumina_evr.read() {
        let lumina_entity = commands
            .spawn((
                LuminaType::Normal.config_info(),
                SpawnBlueprint,
                SourceEntity,
                GlobalTransform::default(),
                Transform::from_xyz(event.position.x, event.position.y, 0.1),
                CollisionLayers::new(GameLayer::Lumina, GameLayer::Spaceship),
                CollidingEntities::default(),
                Sensor,
                RigidBody::Static,
                Visibility::Visible,
                Name::new("Lumina"),
            ))
            .id();

        info!(
            "Spawned Lumina entity {:?} at position {:?}",
            lumina_entity, event.position
        );
    }
}

/// Handles both collision detection and gameplay effects for Lumina collection.
fn lumina_collection(
    mut commands: Commands,
    q_luminas: Query<(&CollidingEntities, Entity, &Transform), With<LuminaStat>>,
    q_players: Query<&PlayerId>,
    mut collected_luminas: ResMut<CollectedLuminas>,
) {
    for (colliding_entities, lumina_entity, _lumina_transform) in q_luminas.iter() {
        for &collided_entity in colliding_entities.iter() {
            if let Ok(player_id) = q_players.get(collided_entity) {
                println!(
                    "Player {:?} collected Lumina {:?}",
                    player_id, lumina_entity
                );

                // Increment pending Luminas for the player.
                *collected_luminas.pending.entry(*player_id).or_insert(0) += 1;

                // Despawn the Lumina.
                commands.entity(lumina_entity).despawn();

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

#[derive(Resource, Default)]
pub struct CollectedLuminas {
    // Luminas collected but not yet deposited.
    pub pending: HashMap<PlayerId, u32>,
}
