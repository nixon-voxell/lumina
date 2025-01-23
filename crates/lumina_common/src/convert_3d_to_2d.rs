use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;

pub(super) struct Convert3dTo2dPlugin;

impl Plugin for Convert3dTo2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorMaterialMap>().add_systems(
            Update,
            (
                convert_3d_to_2d_mesh,
                convert_std_to_color_material,
                material_change_update,
            ),
        );
    }
}

/// Convert all 3d [`Handle<Mesh>`] to 2d [`Mesh2dHandle`].
fn convert_3d_to_2d_mesh(
    mut commands: Commands,
    q_meshes: Query<(&Handle<Mesh>, Option<&Name>, Entity), Changed<Handle<Mesh>>>,
) {
    for (mesh_handle, name, entity) in q_meshes.iter() {
        commands
            .entity(entity)
            .insert(Mesh2dHandle(mesh_handle.clone()));

        debug!("Converted {name:?} 3d mesh into 2d mesh.");
    }
}

/// Convert all [`StandardMaterial`] to [`ColorMaterial`].
fn convert_std_to_color_material(
    mut commands: Commands,
    q_std_materials: Query<
        (&Handle<StandardMaterial>, Option<&Name>, Entity),
        Or<(
            Changed<Handle<StandardMaterial>>,
            Without<Handle<ColorMaterial>>,
        )>,
    >,
    std_materials: Res<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut color_material_map: ResMut<ColorMaterialMap>,
) {
    for (std_material_handle, name, entity) in q_std_materials.iter() {
        match color_material_map.get(&std_material_handle.id()) {
            Some(color_material) => {
                // Reuse the same material handle if a map exists.
                commands.entity(entity).insert(color_material.clone_weak());
            }
            None => {
                // Create a new color material handle if a map does not exists.
                let Some(std_material) = std_materials.get(std_material_handle) else {
                    // We wait for the next update if the material is not ready yet...
                    continue;
                };

                let color_material_handle =
                    color_materials.add(std_to_color_material(std_material));
                let weak_handle = color_material_handle.clone_weak();
                color_material_map.insert(std_material_handle.id(), color_material_handle);

                commands.entity(entity).insert(weak_handle);
            }
        }

        debug!("Converted {name:?} standard material into color material.");
    }
}

/// Update the corresponding [`ColorMaterial`] when a [`StandardMaterial`] is being modified.
fn material_change_update(
    mut std_asset_event_evr: EventReader<AssetEvent<StandardMaterial>>,
    color_material_map: Res<ColorMaterialMap>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    std_materials: Res<Assets<StandardMaterial>>,
) {
    for std_asset_event in std_asset_event_evr.read() {
        if let AssetEvent::Modified { id }
        | AssetEvent::Added { id }
        | AssetEvent::LoadedWithDependencies { id } = std_asset_event
        {
            let Some(std_material) = std_materials.get(*id) else {
                return;
            };

            if let Some(color_material) = color_material_map
                .get(id)
                .and_then(|handle| color_materials.get_mut(handle))
            {
                *color_material = std_to_color_material(std_material);
            }

            debug!("Updating color material: {id:?}");
        }
    }
}

/// Mapping of corresponding [`StandardMaterial`] to [`ColorMaterial`].
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ColorMaterialMap(HashMap<AssetId<StandardMaterial>, Handle<ColorMaterial>>);

pub fn std_to_color_material(std_material: &StandardMaterial) -> ColorMaterial {
    ColorMaterial {
        color: Color::from(
            std_material.base_color.to_linear() + std_material.emissive.with_alpha(0.0),
        ),
        texture: std_material.base_color_texture.clone(),
    }
}
