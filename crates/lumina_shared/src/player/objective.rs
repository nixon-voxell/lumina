use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::health::Health;
use crate::player::PlayerId;
use crate::prelude::{LuminaType, OreType};

use super::GameLayer;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LuminaCollected>()
            .add_systems(Update, (ore_visibility, setup_lumina_col_layer));
    }
}

/// Disable ore visibility & physics if health is 0.0 or below and vice versa.
fn ore_visibility(
    // mut commands: Commands,
    mut q_ores: Query<(&mut Visibility, &Health), (With<OreType>, With<SourceEntity>)>,
) {
    for (mut viz, health) in q_ores.iter_mut() {
        match **health <= 0.0 {
            true => {
                // TODO: Replace with some dulling effect instead.
                viz.set_if_neq(Visibility::Hidden);
            }
            false => {
                viz.set_if_neq(Visibility::Inherited);
            }
        }
    }
}

/// Setup lumina collision layer such that it can only collide with spaceships.
/// (Particularly avoiding ammos)
fn setup_lumina_col_layer(
    mut commands: Commands,
    q_luminas: Query<Entity, (With<LuminaType>, Added<SourceEntity>)>,
) {
    for entity in q_luminas.iter() {
        commands.entity(entity).insert(CollisionLayers::new(
            GameLayer::Lumina,
            [GameLayer::Spaceship],
        ));
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
pub struct LuminaSpawnArea {
    pub radius: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ObjectiveArea {
    /// Ores that are not mined yet ([Health] is still greater than 0.0)
    /// will stay in unused pool and vice versa.
    #[reflect(ignore)]
    pub ores: EntityPool,
}
