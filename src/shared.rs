//! This module contains the shared code between the client and the server.
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::Duration};
use blenvy::BlenvyPlugin;
use input::MovementSet;
use lightyear::prelude::*;

pub const FIXED_TIMESTEP_HZ: f64 = 60.0;
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

        app.configure_sets(
            FixedUpdate,
            (MovementSet::Input, MovementSet::Physics).chain(),
        );

        app.add_plugins((
            crate::protocol::ProtocolPlugin,
            crate::ui::UiPlugin,
            player::PlayerPlugin,
            physics::PhysicsPlugin,
            effector::EffectorPlugin,
        ))
        .add_systems(
            Update,
            (convert_3d_to_2d_mesh, convert_std_to_color_material),
        );
    }
}

/// Convert all 3d [`Handle<Mesh>`] to 2d [`Mesh2dHandle`].
fn convert_3d_to_2d_mesh(mut commands: Commands, q_meshes: Query<(&Handle<Mesh>, &Name, Entity)>) {
    for (mesh_handle, name, entity) in q_meshes.iter() {
        commands
            .entity(entity)
            .remove::<Handle<Mesh>>()
            .insert(Mesh2dHandle(mesh_handle.clone()));

        info!("Converted {name:?} 3d mesh into 2d mesh.");
    }
}

/// Convert all [`StandardMaterial`] to [`ColorMaterial`].
fn convert_std_to_color_material(
    mut commands: Commands,
    q_meshes: Query<(&Handle<StandardMaterial>, &Name, Entity)>,
    std_materials: Res<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (std_material, name, entity) in q_meshes.iter() {
        let Some(std_material) = std_materials.get(std_material) else {
            continue;
        };

        let color_material = color_materials.add(ColorMaterial {
            color: Color::from(std_material.base_color.to_linear() + std_material.emissive),
            texture: std_material.base_color_texture.clone(),
        });

        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>()
            .insert(color_material);

        info!("Converted {name:?} standard material into color material.");
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
