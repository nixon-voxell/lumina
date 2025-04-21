use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

use crate::player::PlayerId;
use crate::prelude::LuminaType;

use super::GameLayer;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LuminaCollected>()
            .observe(setup_lumina_col_layer);
    }
}

/// Setup lumina collision layer such that it can only collide with spaceships.
/// (Particularly avoiding ammos)
fn setup_lumina_col_layer(trigger: Trigger<OnAdd, LuminaType>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).insert(CollisionLayers::new(
        GameLayer::Lumina,
        [GameLayer::Spaceship, GameLayer::Lumina],
    ));
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
pub struct CollectedLumina(pub u8);

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
    /// Ores that are not mined yet ([`Health`] is still greater than 0.0)
    /// will stay in unused pool and vice versa.
    #[reflect(ignore)]
    pub ores: EntityPool,
}
