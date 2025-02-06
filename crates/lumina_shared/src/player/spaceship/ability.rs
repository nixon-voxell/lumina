use bevy::prelude::*;
use lightyear::prelude::*;

pub(super) struct SpaceshipAbilityPlugin;

impl Plugin for SpaceshipAbilityPlugin {
    fn build(&self, _app: &mut App) {
        //
    }
}

type ShadowAbilityConfig = AbilityConfig<ShadowAbility>;
type HealAbilityConfig = AbilityConfig<HealAbility>;

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
pub struct ShadowAbility;

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct HealAbility;
