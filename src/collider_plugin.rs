use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_rapier2d::prelude::*;

use crate::components::NeedsCollider;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_collider_to_entity);
    }
}

fn add_collider_to_entity(
    mut commands: Commands,
    query: Query<(Entity, &mut Mesh2d), With<NeedsCollider>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, mesh_handle) in query.iter() {
        if let Some(mesh) = meshes.get(&*mesh_handle) {
            if let Some(vertices) = extract_vertices_from_mesh(mesh) {
                if let Some(collider) = Collider::convex_hull(&vertices) {
                    commands.entity(entity).insert(collider);
                }
            }
        }
    }
}

fn extract_vertices_from_mesh(mesh: &Mesh) -> Option<Vec<Vec2>> {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION.id)
    {
        let vertices: Vec<Vec2> = positions.iter().map(|&[x, y, _]| Vec2::new(x, y)).collect();
        Some(vertices)
    } else {
        None
    }
}