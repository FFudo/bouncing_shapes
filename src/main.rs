use bevy::{prelude::*, window::*};
use rand::Rng;

const MAX_SHAPES: i32 = 5;

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bouncing Shapes".to_string(),
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_shapes)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    window_query: Query<&Window>,
) {
    let mut rng = rand::thread_rng();

    let width: f32;
    if let Ok(window) = window_query.get_single() {
        width = window.width();
    } else {
        width = 1.0;
    }

    commands.spawn(Camera2d::default());

    let mut shapes = Vec::new();

    for _ in 0..MAX_SHAPES {
        match rng.gen_range(0..3) {
            0 => {
                shapes.push(meshes.add(Circle::new(50.0)));
            }
            1 => {
                shapes.push(meshes.add(Rectangle::new(50.0, 100.0)));
            }
            2 => {
                shapes.push(meshes.add(Annulus::new(25.0, 50.0)));
            }
            _ => {
                shapes.push(meshes.add(Rhombus::new(75.0, 100.0)));
            }
        }
    }

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / MAX_SHAPES as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(50. - (width / 2 as f32) + (i * 120) as f32, 0.0, 0.0),
            Shape,
        ));
    }
}

fn move_shapes(mut query: Query<&mut Transform, With<Shape>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
        transform.translation.y += 1.0;
    }
}
