use crate::components::*;
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_rapier2d::prelude::*;
use rand::*;
pub struct SpawnShapesPlugin;

const MAX_SHAPES: i32 = 5;
const CIRCLE_SIZE: f32 = 50.0;
const RECTANGLE_SIZE: (f32, f32) = (100.0, 75.0);

impl Plugin for SpawnShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_shapes, add_collider_to_entity).chain());
    }
}

fn spawn_shapes(
    mut materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    let shapes = get_random_shapes(meshes);

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / MAX_SHAPES as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape.0),
            MeshMaterial2d(materials.add(color)),
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            Transform::from_xyz(
                rng.gen_range(-200..200) as f32,
                rng.gen_range(-200..200) as f32,
                rng.gen_range(0..=MAX_SHAPES) as f32,
            ),
            NeedsCollider,
            shape.1,
            Ccd::enabled(),
            ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0
            }
        ));
    }
}

fn get_random_shapes(
    mut meshes: ResMut<'_, Assets<Mesh>>,
) -> Vec<(Handle<Mesh>, ShapeType, Collider)> {
    let mut shapes = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..MAX_SHAPES {
        match rng.gen_range(0..=5) {
            0 => {
                shapes.push((
                    meshes.add(Circle::new(CIRCLE_SIZE)),
                    ShapeType::Circle(CIRCLE_SIZE),
                    Collider::ball(CIRCLE_SIZE),
                ));
            }
            1 => {
                shapes.push((
                    meshes.add(Rectangle::new(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1)),
                    ShapeType::Rectangle(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1),
                    Collider::cuboid(RECTANGLE_SIZE.0 / 2.0, RECTANGLE_SIZE.1 / 2.0),
                ));
            }
            2 => {
                shapes.push((
                    meshes.add(Annulus::new(CIRCLE_SIZE / 2.0, CIRCLE_SIZE)),
                    ShapeType::Annulus(CIRCLE_SIZE / 2.0, CIRCLE_SIZE),
                    Collider::ball(CIRCLE_SIZE),
                ));
            }
            3 => {
                shapes.push((
                    meshes.add(Rhombus::new(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1)),
                    ShapeType::Rhombus(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1),
                    Collider::cuboid(RECTANGLE_SIZE.0 / 2.0, RECTANGLE_SIZE.1 / 2.0),
                ));
            }
            4 => {
                shapes.push((
                    meshes.add(RegularPolygon::new(CIRCLE_SIZE, 12)),
                    ShapeType::RegularPolygon(CIRCLE_SIZE, 12),
                    Collider::ball(CIRCLE_SIZE),
                ));
            }
            5 => {
                shapes.push((
                    meshes.add(Triangle2d::new(
                        Vec2::Y * 50.0,
                        Vec2::new(-50.0, -50.0),
                        Vec2::new(50.0, -50.0),
                    )),
                    ShapeType::Triangle(
                        Vec2::Y * 50.0,
                        Vec2::new(-50.0, -50.0),
                        Vec2::new(50.0, -50.0),
                    ),
                    Collider::triangle(
                        Vec2::Y * 50.0,
                        Vec2::new(-50.0, -50.0),
                        Vec2::new(50.0, -50.0),
                    ),
                ));
            }
            _ => {}
        }
    }
    return shapes;
}


fn add_collider_to_entity(
    mut commands: Commands,
    query: Query<(Entity, &mut Mesh2d), With<NeedsCollider>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, mesh_handle) in query.iter() {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {
            if let Some(vertices) = extract_vertices_from_mesh(mesh) {
                if let Some(collider) = Collider::convex_hull(&vertices) {
                    commands.entity(entity).insert(collider);
                }
            }
        }
    }
}


fn extract_vertices_from_mesh(mesh: &Mesh) -> Option<Vec<Vec2>> {
    if let Some(VertexAttributeValues::Float32x3(positions) ) = mesh.attribute(Mesh::ATTRIBUTE_POSITION.id) {
        let vertices: Vec<Vec2> = positions
        .iter()
        .map(|&[x, y, _]| Vec2::new(x, y))
        .collect();
        Some(vertices)
    } else {
        None
    }
}
