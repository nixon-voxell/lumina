use bevy::prelude::*;

pub mod blueprint_visual;
pub mod convert_3d_to_2d;
pub mod physics;
pub mod settings;
pub mod source_entity;
pub mod utils;

pub mod prelude {
    pub use crate::blueprint_visual::{BlueprintType, SpawnBlueprintVisualAppExt};
    pub use crate::settings::LuminaSettings;
    pub use crate::source_entity::{SetSourceAppExt, SourceEntity};
    pub use crate::utils::{propagate_component, EntityRoomId, TransformSyncSet};
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
        ));
    }
}
