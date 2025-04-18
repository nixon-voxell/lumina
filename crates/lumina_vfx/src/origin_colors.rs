use std::marker::PhantomData;

use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use lumina_common::prelude::*;

pub struct OriginColorsPlugin<Filter: QueryFilter>(PhantomData<Filter>);

impl<Filter: QueryFilter + ThreadSafe> Plugin for OriginColorsPlugin<Filter> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_origin_colors::<Filter>.after(Convert3dTo2dSet));
    }
}

impl<Filter: QueryFilter> Default for OriginColorsPlugin<Filter> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Initialize the original colors of spaceship materials
/// within the hierarchy of the filter criteria.
fn init_origin_colors<Filter: QueryFilter>(
    mut commands: Commands,
    q_entities: Query<Entity, Filter>,
    q_children: Query<&Children>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in q_entities.iter() {
        // Initialize origin colors of the materials.
        let mut origin_colors = OriginColors::default();
        let entity_color_pairs = q_children
            .iter_descendants(entity)
            .filter_map(|e| {
                q_color_materials
                    .get(e)
                    .ok()
                    .and_then(|handle| color_materials.get(handle))
                    .map(|color_material| (e, color_material.clone()))
            })
            .collect::<Vec<_>>();

        for (child, color_material) in entity_color_pairs {
            origin_colors.push((child, color_material.color));
            // Create a new instance of the material so that it would only affect
            // this specific instance instead of being shared with other materials.
            commands
                .entity(child)
                .insert(color_materials.add(color_material));
        }

        commands.entity(entity).insert(origin_colors);
        info!("Setup origin colors for {entity}");
    }
}

/// Original color of the materials inside a spaceship hierarchy.
#[derive(Component, Deref, DerefMut, Default, Debug, Clone)]
pub struct OriginColors(Vec<(Entity, Color)>);
