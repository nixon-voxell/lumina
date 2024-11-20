use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use blenvy::*;

use crate::prelude::SourceEntity;

pub trait SpawnBlueprintVisualAppExt {
    fn spawn_blueprint_visual<T: BlueprintType, F: QueryFilter + 'static>(&mut self) -> &mut Self;
}

impl SpawnBlueprintVisualAppExt for App {
    fn spawn_blueprint_visual<T: BlueprintType, F: QueryFilter + 'static>(&mut self) -> &mut Self {
        self.add_systems(Update, spawn_blueprint_visual_impl::<T, F>)
    }
}

fn spawn_blueprint_visual_impl<T: BlueprintType, F: QueryFilter>(
    mut commands: Commands,
    q_blueprints: Query<(&T, Entity), (Added<SourceEntity>, F)>,
) {
    for (blueprint_type, entity) in q_blueprints.iter() {
        commands.entity(entity).insert((
            blueprint_type.visual_info(),
            SpawnBlueprint,
            HideUntilReady,
        ));
    }
}

pub trait BlueprintType: Component {
    fn info(&self) -> BlueprintInfo;
    fn visual_info(&self) -> BlueprintInfo;
    fn config_info(&self) -> BlueprintInfo;
}

impl<T: AsRef<str> + Component> BlueprintType for T {
    fn info(&self) -> BlueprintInfo {
        let mut name = self.as_ref().to_string();
        name += ".glb";
        BlueprintInfo::from_path(&name)
    }

    fn visual_info(&self) -> BlueprintInfo {
        let mut name = self.as_ref().to_string();
        name += "Visual.glb";
        BlueprintInfo::from_path(&name)
    }

    fn config_info(&self) -> BlueprintInfo {
        let mut name = self.as_ref().to_string();
        name += "Config.glb";
        BlueprintInfo::from_path(&name)
    }
}
