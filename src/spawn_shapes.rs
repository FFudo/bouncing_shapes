use crate::components::*;
use bevy::prelude::*;
use rand::*;

pub struct SpawnShapesPlugin;

const MAX_SHAPES: i32 = 6;
const CIRCLE_SIZE: f32 = 50.0;
const RECTANGLE_SIZE: (f32, f32) = (100.0, 75.0);

impl Plugin for SpawnShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shapes);
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
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                rng.gen_range(-200..200) as f32,
                rng.gen_range(-200..200) as f32,
                i as f32,
            ),
            Impuls,
            Velocity {
                velocity: Vec2::ZERO,
                friction: 0.9,
            },
        ));
    }
}

fn get_random_shapes(mut meshes: ResMut<'_, Assets<Mesh>>) -> Vec<Handle<Mesh>> {
    let mut shapes = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..MAX_SHAPES {
        match rng.gen_range(0..=5) {
            0 => {
                shapes.push(meshes.add(Circle::new(CIRCLE_SIZE)));
            }
            1 => {
                shapes.push(meshes.add(Rectangle::new(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1)));
            }
            2 => {
                shapes.push(meshes.add(Annulus::new(CIRCLE_SIZE / 2.0, CIRCLE_SIZE)));
            }
            3 => {
                shapes.push(meshes.add(Rhombus::new(RECTANGLE_SIZE.0, RECTANGLE_SIZE.1)));
            }
            4 => {
                shapes.push(meshes.add(RegularPolygon::new(CIRCLE_SIZE, 12)));
            }
            5 => {
                shapes.push(meshes.add(Triangle2d::new(
                    Vec2::Y * 50.0,
                    Vec2::new(-50.0, -50.0),
                    Vec2::new(50.0, -50.0),
                )));
            }
            _ => {}
        }
    }
    return shapes;
}
