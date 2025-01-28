use bevy::{prelude::*, window::*};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::Mutex;

mod components;
use crate::components::*;

const MAX_SHAPES: i32 = 6;

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 720.0;
const CIRCLE_SIZE: f32 = 50.0;
const RECTANGLE_SIZE: (f32, f32) = (100.0, 75.0);

fn main() {
    let seed = rand::thread_rng().gen();
    let rng = StdRng::from_seed(seed);

    App::new()
        .insert_resource(ImpulsTimer(Timer::from_seconds(2.5, TimerMode::Repeating)))
        .insert_resource(RngResource(Mutex::new(rng)))
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
        .add_systems(
            Update,
            (
                apply_impuls,
                apply_friction,
                apply_wall_collision,
                apply_velocity,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    rng: Res<RngResource>,
) {
    commands.spawn(Camera2d::default());

    let mut rng = rng.0.lock().unwrap();
    let mut shapes = Vec::new();

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

fn apply_impuls(
    time: Res<Time>,
    rng: Res<RngResource>,
    mut timer: ResMut<ImpulsTimer>,
    mut query: Query<&mut Velocity, With<Impuls>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rng.0.lock().unwrap();
        for mut velocity in query.iter_mut() {
            if rng.gen_bool(0.4) {
                let dir_x = if rng.gen_bool(0.5) { 1 } else { -1 } as f32;
                let dir_y = if rng.gen_bool(0.5) { 1 } else { -1 } as f32;
                let impuls = Vec2::new(
                    rng.gen_range(0..300) as f32 * dir_x,
                    rng.gen_range(0..300) as f32 * dir_y,
                );
                velocity.velocity = impuls;
            }
        }
    }
}

fn apply_friction(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        let friction = velocity.friction;
        velocity.velocity *= friction.powf(time.delta_secs());

        if velocity.velocity.length() <= 0.01 {
            velocity.velocity = Vec2::ZERO;
        }
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.velocity.extend(0.0) * time.delta_secs();
    }
}

fn apply_wall_collision(mut query: Query<(&mut Velocity, &Transform)>) {
    for (mut velocity, transform) in query.iter_mut() {
        if transform.translation.x > WINDOW_WIDTH / 2.0
            || transform.translation.x < -WINDOW_WIDTH / 2.0
        {
            velocity.velocity.x *= -1.0;
        }
        if transform.translation.y > WINDOW_HEIGHT / 2.0
            || transform.translation.y < -WINDOW_HEIGHT / 2.0
        {
            velocity.velocity.y *= -1.0;
        }
    }
}
