use avian2d::math::Scalar;
use avian2d::parry::na::Point2;
use avian2d::parry::shape::SharedShape;
use avian2d::prelude::*;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::sprite::Mesh2dHandle;
use lightyear::prelude::*;

use crate::settings::LuminaSettings;

pub mod physics_interp;
pub mod world;

pub(super) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::new(FixedPostUpdate)
                // PhysicsPlugins::default()
                // 1 pixel is 10 units
                .with_length_unit(10.0),
            // #[cfg(feature = "dev")]
            // PhysicsDebugPlugin::default(),
        ))
        .add_plugins((
            physics_interp::PhysicsInterpPlugin,
            world::PhysicsWorldPlugin,
        ));

        let settings = app.world().get_resource::<LuminaSettings>().unwrap();
        let fixed_timestep_hz = settings.fixed_timestep_hz;

        app.insert_resource(Time::<Fixed>::from_hz(fixed_timestep_hz))
            .insert_resource(Time::new_with(Physics::fixed_once_hz(fixed_timestep_hz)))
            .insert_resource(Gravity(Vec2::ZERO))
            // Physics determines the transform of an entity.
            .insert_resource(avian2d::sync::SyncConfig {
                transform_to_position: false,
                position_to_transform: true,
            })
            .add_systems(Last, (convert_primitive_rigidbody, convert_mesh_rigidbody));
    }
}

fn convert_primitive_rigidbody(
    mut commands: Commands,
    q_rigidbodies: Query<(&PrimitiveRigidbody, Entity), Added<PrimitiveRigidbody>>,
) {
    for (rigidbody, entity) in q_rigidbodies.iter() {
        match Collider::try_from_constructor(rigidbody.collider_constructor.clone()) {
            Some(collider) => {
                commands.entity(entity).insert((
                    MassPropertiesBundle::new_computed(&collider, *rigidbody.density),
                    collider,
                    rigidbody.rigidbody,
                ));

                debug!("Generated primitive collider for {entity}.")
            }
            None => error!("Unable to convert ColliderConstructor into Collider for {entity}."),
        }

        // commands.entity(entity).remove::<PrimitiveRigidbody>();
    }
}

fn convert_mesh_rigidbody(
    mut commands: Commands,
    q_rigidbodies: Query<
        (
            &MeshRigidbody,
            Option<&Mesh2dHandle>,
            Option<&Handle<Mesh>>,
            Entity,
        ),
        Added<MeshRigidbody>,
    >,
    meshes: Res<Assets<Mesh>>,
) {
    for (rigidbody, mesh2d, mesh3d, entity) in q_rigidbodies.iter() {
        let Some(mesh_handle) = mesh3d.or(mesh2d.map(|mesh2d| &**mesh2d)) else {
            warn!("Configured with Trimesh collider but wasn't attached with any Mesh.");
            commands.entity(entity).remove::<MeshRigidbody>();
            continue;
        };

        let Some(mesh) = meshes.get(mesh_handle) else {
            // Early continue if mesh is not available yet.
            continue;
        };

        // Generate collider from mesh
        let collider = match rigidbody.collider_type {
            MeshCollider::ConvexHull => convex_hull_from_mesh(mesh),
            MeshCollider::Trimesh => trimesh_from_mesh(mesh),
        };

        match collider {
            Some(collider) => {
                commands.entity(entity).insert((
                    MassPropertiesBundle::new_computed(&collider, *rigidbody.density),
                    collider,
                    rigidbody.rigidbody,
                ));

                debug!("Generated mesh collider for {entity}.")
            }
            None => error!("Unable to generate Collider from Mesh for {entity}."),
        }

        // commands.entity(entity).remove::<MeshRigidbody>();
    }
}

type VerticesIndices = (Vec<Point2<Scalar>>, Vec<[u32; 3]>);

pub fn extract_mesh_vertices_indices(mesh: &Mesh) -> Option<VerticesIndices> {
    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let indices = mesh.indices()?;

    let vertices = match vertices {
        VertexAttributeValues::Float32(vtx) => {
            Some(vtx.chunks(3).map(|v| [v[0], v[1]].into()).collect())
        }
        VertexAttributeValues::Float32x3(vtx) => {
            Some(vtx.iter().map(|v| [v[0], v[1]].into()).collect())
        }
        _ => None,
    }?;

    let indices = match indices {
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
        Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
    };

    Some((vertices, indices))
}

pub fn trimesh_from_mesh(mesh: &Mesh) -> Option<Collider> {
    extract_mesh_vertices_indices(mesh).map(|(vertices, indices)| {
        SharedShape::trimesh_with_flags(
            vertices,
            indices,
            TrimeshFlags::MERGE_DUPLICATE_VERTICES.into(),
        )
        .into()
    })
}

pub fn trimesh_from_mesh_with_config(mesh: &Mesh, flags: TrimeshFlags) -> Option<Collider> {
    extract_mesh_vertices_indices(mesh).map(|(vertices, indices)| {
        SharedShape::trimesh_with_flags(vertices, indices, flags.into()).into()
    })
}

pub fn convex_hull_from_mesh(mesh: &Mesh) -> Option<Collider> {
    extract_mesh_vertices_indices(mesh)
        .and_then(|(vertices, _)| SharedShape::convex_hull(&vertices).map(|shape| shape.into()))
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PrimitiveRigidbody {
    pub rigidbody: RigidBody,
    pub density: ColliderDensity,
    pub collider_constructor: ColliderConstructor,
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct MeshRigidbody {
    pub rigidbody: RigidBody,
    pub density: ColliderDensity,
    pub collider_type: MeshCollider,
}

#[derive(Reflect, Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshCollider {
    #[default]
    ConvexHull,
    Trimesh,
}

pub trait RemovePhysicsCreatorAppExt {
    fn remove_physics_creator<F: QueryFilter + 'static>(&mut self) -> &mut Self;
}

impl RemovePhysicsCreatorAppExt for App {
    fn remove_physics_creator<F: QueryFilter + 'static>(&mut self) -> &mut Self {
        self.add_systems(PostUpdate, remove_physics_creator_impl::<F>)
    }
}

/// Remove [PrimitiveRigidbody] & [MeshRigidbody] for the given criteria before their creation.
fn remove_physics_creator_impl<F: QueryFilter>(
    mut commands: Commands,
    q_entities: Query<Entity, (F, Or<(With<PrimitiveRigidbody>, With<MeshRigidbody>)>)>,
) {
    for entity in q_entities.iter() {
        commands
            .entity(entity)
            .remove::<(PrimitiveRigidbody, MeshRigidbody)>();
    }
}
