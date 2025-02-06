use crate::{components::*, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::*;
pub struct SpawnShapesPlugin;

const MAX_SHAPES: i32 = 5;
const CIRCLE_SIZE: f32 = 50.0;
const RECTANGLE_SIZE: (f32, f32) = (100.0, 75.0);

impl Plugin for SpawnShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_shapes, spawn_walls).chain(),
        );
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
            Ccd::enabled(),
            ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            },
            Restitution::coefficient(0.9),
            Friction::coefficient(0.0)
        ));
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let wall_color = Color::hsl(360.0, 0.1, 0.1);
    let wall_positions: [(f32, f32, bool); 4] = [
        (0.0, WINDOW_HEIGHT / 2.0, false),     // Top wall
        (0.0, -WINDOW_HEIGHT / 2.0, false),    // Bottom wall
        (WINDOW_WIDTH / 2.0, 0.0, true),       // Right wall
        (-WINDOW_WIDTH / 2.0, 0.0, true),      // Left wall
    ];

    for (x, y, is_vertical) in wall_positions {
        let (width, height) = if is_vertical {
            (50.0, WINDOW_HEIGHT)
        } else {
            (WINDOW_WIDTH, 50.0)
        };

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(wall_color))),
            NeedsCollider,
            Transform::from_xyz(x, y, 1.0),
            RigidBody::Fixed
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

