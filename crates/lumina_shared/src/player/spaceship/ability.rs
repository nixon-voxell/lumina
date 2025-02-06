use bevy::prelude::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;

pub(super) struct SpaceshipAbilityPlugin;

impl Plugin for SpaceshipAbilityPlugin {
    fn build(&self, _app: &mut App) {
        //
    }
}

fn init_shadow_ability(mut commands: Commands, q_configs: Query<Entity, Added<SourceEntity>>) {}

pub type ShadowAbilityConfig = AbilityConfig<ShadowAbility>;
pub type HealAbilityConfig = AbilityConfig<HealAbility>;

/// Configuration of an ability.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct AbilityConfig<T> {
    cooldown: f32,
    ability: T,
}

/// Cooldown timer based on [`AbilityConfig::cooldown`].
/// While this component is still in effect, [ability action][crate::action::PlayerAction::Ability] cannot be used.
#[derive(Component, Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq)]
pub struct AbilityCooldown(Timer);

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct ShadowAbility {
    /// A color multiplier for the spaceship's material. (Should be a negative value).
    pub strength: f32,
    /// Duration of the ability in seconds.
    pub duration: f32,
}

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct HealAbility {
    /// Radius of the ability.
    pub radius: f32,
    /// Duration of the ability in seconds.
    pub duration: f32,
}
