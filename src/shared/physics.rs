use avian2d::{
    math::Scalar,
    parry::{na::Point2, shape::SharedShape},
    prelude::*,
};
use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
    sprite::Mesh2dHandle,
};

use super::FIXED_TIMESTEP_HZ;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // We change SyncPlugin to PostUpdate, because we want the visually interpreted values
        // synced to transform every time, not just when Fixed schedule runs.
        app.add_plugins(
            PhysicsPlugins::new(FixedPostUpdate)
                // 1 pixel is 10 units
                .with_length_unit(10.0),
        );

        app.insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_HZ))
            .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
            .insert_resource(Gravity(Vec2::ZERO))
            // We use Position and Rotation as primary source of truth, so no need to sync changes
            // from Transform->Pos, just Pos->Transform.
            .insert_resource(avian2d::sync::SyncConfig {
                transform_to_position: false,
                position_to_transform: true,
            })
            .add_systems(
                Update,
                (convert_primitive_rigidbody, convert_mesh_rigidbody),
            );

        app.register_type::<PrimitiveRigidbody>()
            .register_type::<MeshRigidbody>();
    }
}

fn convert_primitive_rigidbody(
    mut commands: Commands,
    q_rigidbodies: Query<(&PrimitiveRigidbody, Entity)>,
) {
    for (rigidbody, entity) in q_rigidbodies.iter() {
        match Collider::try_from_constructor(rigidbody.collider_constructor.clone()) {
            Some(collider) => {
                commands.entity(entity).insert((
                    MassPropertiesBundle::new_computed(&collider, *rigidbody.density),
                    collider,
                    rigidbody.rigidbody,
                ));

                info!("Generated primitive collider for {entity:?}.")
            }
            None => error!("Unable to convert ColliderConstructor into Collider."),
        }

        commands.entity(entity).remove::<PrimitiveRigidbody>();
    }
}

fn convert_mesh_rigidbody(
    mut commands: Commands,
    q_rigidbodies: Query<(
        &MeshRigidbody,
        Option<&Mesh2dHandle>,
        Option<&Handle<Mesh>>,
        Entity,
    )>,
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

                info!("Generated mesh collider for {entity:?}.")
            }
            None => error!("Unable to generate Collider from Mesh."),
        }

        commands.entity(entity).remove::<MeshRigidbody>();
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

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component, Debug)]
pub struct PrimitiveRigidbody {
    pub rigidbody: RigidBody,
    pub density: ColliderDensity,
    pub collider_constructor: ColliderConstructor,
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component, Default, Debug)]
pub struct MeshRigidbody {
    pub rigidbody: RigidBody,
    pub density: ColliderDensity,
    pub collider_type: MeshCollider,
}

#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Default, Debug)]
pub enum MeshCollider {
    #[default]
    ConvexHull,
    Trimesh,
}
