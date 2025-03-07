use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

pub(super) struct TeleporterPlugin;

impl Plugin for TeleporterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CooldownEffectPlugin::<Teleporter, TeleporterStart>::default())
            .add_systems(Update, propagate_component::<Teleporter>);
    }
}

pub type TeleporterEffect = Effect<Teleporter>;
pub type TeleporterCooldown = Cooldown<Teleporter>;

/// The starting point of the teleporter.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TeleporterStart {
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

/// The end point of the teleporter.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TeleporterEnd;

/// The unique ID of the teleporter. Start and end points
/// of the teleporte should have the same ID.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct Teleporter(pub u32);
