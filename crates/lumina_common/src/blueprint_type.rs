use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use blenvy::*;

use crate::prelude::SourceEntity;

pub trait SpawnBlueprintTypeAppExt {
    fn spawn_blueprint_visual<T: BlueprintType, F: QueryFilter + 'static>(&mut self) -> &mut Self;
    fn spawn_blueprint_collider<T: BlueprintType, F: QueryFilter + 'static>(&mut self)
        -> &mut Self;
}

impl SpawnBlueprintTypeAppExt for App {
    fn spawn_blueprint_visual<T: BlueprintType, F: QueryFilter + 'static>(&mut self) -> &mut Self {
        self.add_systems(Update, spawn_blueprint_visual_impl::<T, F>)
    }

    fn spawn_blueprint_collider<T: BlueprintType, F: QueryFilter + 'static>(
        &mut self,
    ) -> &mut Self {
        self.add_systems(Update, spawn_blueprint_collider_impl::<T, F>)
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

fn spawn_blueprint_collider_impl<T: BlueprintType, F: QueryFilter>(
    mut commands: Commands,
    q_blueprints: Query<(&T, Entity), (Added<SourceEntity>, F)>,
) {
    for (blueprint_type, entity) in q_blueprints.iter() {
        commands.entity(entity).with_children(|builder| {
            builder.spawn((
                blueprint_type.collider_info(),
                SpawnBlueprint,
                HideUntilReady,
            ));
        });
    }
}

pub trait BlueprintType: AsRef<str> + Component {
    fn info(&self) -> BlueprintInfo {
        self.create_info(".glb")
    }

    fn visual_info(&self) -> BlueprintInfo {
        self.create_info("Visual.glb")
    }

    fn config_info(&self) -> BlueprintInfo {
        self.create_info("Config.glb")
    }

    fn collider_info(&self) -> BlueprintInfo {
        self.create_info("Collider.glb")
    }

    fn create_info(&self, suffix: &str) -> BlueprintInfo {
        let mut name = self.as_ref().to_string();
        name += suffix;
        BlueprintInfo::from_path(&name)
    }
}

impl<T: AsRef<str> + Component> BlueprintType for T {}
