use bevy::prelude::*;
use blenvy::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

pub(super) struct TeleporterPlugin;

impl Plugin for TeleporterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CooldownEffectPlugin::<Teleporter, TeleporterStart>::default())
            .add_systems(
                Update,
                setup_teleporters.in_set(GltfBlueprintsSet::AfterSpawn),
            );
    }
}

fn setup_teleporters(
    mut q_teleporter_starts: Query<(&mut TeleporterStart, Entity), Added<TeleporterStart>>,
    q_teleporter_ends: Query<(), With<TeleporterEnd>>,
    q_parents: Query<&Parent>,
) {
    for (mut teleporter_start, start_entity) in q_teleporter_starts.iter_mut() {
        for parent in q_parents.iter_ancestors(start_entity) {
            if q_teleporter_ends.contains(parent) {
                teleporter_start.end = Some(parent);
                break;
            }
        }
    }
}

pub type TeleporterEffect = Effect<Teleporter>;
pub type TeleporterCooldown = Cooldown<Teleporter>;

/// Marker for teleporter cooldown effect.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Teleporter;

/// The starting point of the teleporter.
/// This needs to be in the child hierarchy of a [`TeleporterEnd`].
/// A teleporter can have multiple starting points but only one end point.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TeleporterStart {
    #[reflect(ignore)]
    end: Option<Entity>,
    /// How long it stays active until cooldown happens.
    active_duration: f32,
    cooldown_duration: f32,
}

impl CooldownEffectConfig for TeleporterStart {
    fn effect_duration(&self) -> f32 {
        self.active_duration
    }

    fn cooldown_duration(&self) -> f32 {
        self.cooldown_duration
    }
}

impl TeleporterStart {
    pub fn end(&self) -> Option<Entity> {
        self.end
    }
}

/// The end point of the teleporter that holds the teleporter ID.
/// Each teleporter should hold a unique ID.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct TeleporterEnd(u32);
