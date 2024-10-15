//! This module contains the shared code between the client and the server.
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::{Duration, HashMap};
use blenvy::BlenvyPlugin;
use lightyear::prelude::*;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub mod effector;
pub mod input;
pub mod physics;
pub mod player;

/// Shared logic.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlenvyPlugin::default());

        app.init_resource::<ColorMaterialMap>()
            .add_plugins((
                crate::protocol::ProtocolPlugin,
                crate::ui::UiPlugin,
                player::PlayerPlugin,
                physics::PhysicsPlugin,
                effector::EffectorPlugin,
            ))
            .add_systems(
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
    q_meshes: Query<(&Handle<Mesh>, Option<&Name>, Entity)>,
) {
    for (mesh_handle, name, entity) in q_meshes.iter() {
        commands
            .entity(entity)
            .remove::<Handle<Mesh>>()
            .insert(Mesh2dHandle(mesh_handle.clone()));

        if let Some(name) = name {
            info!("Converted {name:?} 3d mesh into 2d mesh.");
        }
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
                commands
                    .entity(entity)
                    .insert(Handle::Weak(*color_material));
            }
            None => {
                // Create a new color material handle if a map does not exists.
                let Some(std_material) = std_materials.get(std_material_handle) else {
                    // We wait for the next update if the material is not ready yet...
                    continue;
                };

                let color_material_handle =
                    color_materials.add(std_to_color_material(std_material));
                color_material_map.insert(std_material_handle.id(), color_material_handle.id());

                commands.entity(entity).insert(color_material_handle);
            }
        }

        if let Some(name) = name {
            info!("Converted {name:?} standard material into color material.");
        }
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
        if let AssetEvent::Modified { id } = std_asset_event {
            let Some(std_material) = std_materials.get(*id) else {
                return;
            };

            if let Some(color_material) = color_material_map
                .get(id)
                .and_then(|handle| color_materials.get_mut(*handle))
            {
                *color_material = std_to_color_material(std_material);
            }

            info!("Updating color material: {:?}", id);
        }
    }
}

/// Mapping of corresponding [`StandardMaterial`] to [`ColorMaterial`].
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ColorMaterialMap(HashMap<AssetId<StandardMaterial>, AssetId<ColorMaterial>>);

pub fn std_to_color_material(std_material: &StandardMaterial) -> ColorMaterial {
    ColorMaterial {
        color: Color::from(
            std_material.base_color.to_linear() + std_material.emissive.with_alpha(0.0),
        ),
        texture: std_material.base_color_texture.clone(),
    }
}

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode: Mode::Separate,
    }
}

/// Specify that an entity is not supposed to be networked.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct LocalEntity;
