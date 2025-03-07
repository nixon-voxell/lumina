use bevy::prelude::*;

pub mod asset_from_component;
pub mod blueprint_type;
pub mod convert_3d_to_2d;
pub mod cooldown_effect;
pub mod entity_pool;
pub mod math_utils;
pub mod physics;
pub mod settings;
pub mod source_entity;
pub mod type_registry;
pub mod utils;

pub mod prelude {
    pub use crate::asset_from_component::{AssetFromComponent, AssetFromComponentPlugin};
    pub use crate::blueprint_type::*;
    pub use crate::convert_3d_to_2d::Convert3dTo2dSet;
    pub use crate::cooldown_effect::{
        Cooldown, CooldownEffectCommandExt, CooldownEffectConfig, CooldownEffectPlugin, Effect,
    };
    pub use crate::entity_pool::*;
    pub use crate::math_utils::*;
    pub use crate::physics::world::WorldIdx;
    pub use crate::physics::{MassRigidbody, MeshCollider, MeshRigidbody, PrimitiveRigidbody};
    pub use crate::settings::LuminaSettings;
    pub use crate::source_entity::{SetSourceAppExt, SourceEntity};
    pub use crate::utils::*;
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            utils::UtilsPlugin,
            settings::SettingsPlugin,
            source_entity::SourceEntityPlugin,
            convert_3d_to_2d::Convert3dTo2dPlugin,
            physics::PhysicsPlugin,
            type_registry::TypeRegistryPlugin,
        ));
    }
}
