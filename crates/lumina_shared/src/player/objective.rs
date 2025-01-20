use avian2d::prelude::*;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::blueprints::LuminaType;
use crate::health::Health;
use crate::player::{GameLayer, PlayerId};
use crate::prelude::OreType;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LuminaCollected>()
            .add_systems(Update, (lumina_collision_layer, ore_visibility));
    }
}

/// Initialize lumina's collision layer.
fn lumina_collision_layer(mut commands: Commands, q_lumina: Query<Entity, Added<LuminaType>>) {
    for entity in q_lumina.iter() {
        commands.entity(entity).insert(CollisionLayers::new(
            GameLayer::Lumina,
            GameLayer::Spaceship,
        ));
    }
}

/// Disable ore visibility if health is 0.0 or below and vice versa.
fn ore_visibility(
    mut commands: Commands,
    mut q_ores: Query<(&mut Visibility, &Health, Entity), (With<OreType>, With<SourceEntity>)>,
) {
    for (mut viz, health, entity) in q_ores.iter_mut() {
        match **health <= 0.0 {
            true => {
                if viz.set_if_neq(Visibility::Hidden) {
                    commands.entity(entity).remove::<RigidBody>();
                }
            }
            false => {
                if viz.set_if_neq(Visibility::Inherited) {
                    commands.entity(entity).insert(RigidBody::Static);
                }
            }
        }
    }
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
pub struct CollectedLumina(u8);

impl CollectedLumina {
    /// Max lumina a player can hold.
    pub const MAX: u8 = 15;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LuminaStat {
    pub lifetime: f32,
}

/// Event triggered when Lumina is collected.
#[derive(Event)]
pub struct LuminaCollected {
    pub player_id: PlayerId,
    pub position: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ObjectiveArea {
    /// Area cooldown duration after all luminas have been mined.
    pub cooldown: f32,
    /// Ores that are not mined yet ([Health] is still greater than 0.0).
    #[reflect(ignore)]
    pub unused_ores: EntityHashSet,
    /// Ores that are already being mined ([Health] is lesser or equal to 0.0).
    #[reflect(ignore)]
    pub used_ores: EntityHashSet,
}
