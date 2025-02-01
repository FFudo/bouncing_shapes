use bevy::{prelude::*, window::*};
use rand::*;

mod components;
mod spawn_shapes;

use crate::components::*;
use crate::spawn_shapes::SpawnShapesPlugin;

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ImpulsTimer(Timer::from_seconds(2.5, TimerMode::Repeating)))
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
        .add_plugins(SpawnShapesPlugin)
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
    mut commands: Commands
) {
    commands.spawn(Camera2d::default());
}

fn apply_impuls(
    time: Res<Time>,
    mut timer: ResMut<ImpulsTimer>,
    mut query: Query<&mut Velocity, With<Impuls>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
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

fn apply_wall_collision(mut query: Query<(&mut Velocity, &Transform, &ShapeType)>) {
    for (mut velocity, transform, shapetype) in query.iter_mut() {
        match shapetype {
            ShapeType::Circle(radius) | ShapeType::Annulus(radius, _) | ShapeType::RegularPolygon(radius, _ )=> {
                if transform.translation.x + radius > WINDOW_WIDTH / 2.0
                || transform.translation.x - radius < -WINDOW_WIDTH / 2.0
                {
                    velocity.velocity.x *= -1.0;
                }
                if transform.translation.y + radius > WINDOW_HEIGHT / 2.0
                || transform.translation.y - radius < -WINDOW_HEIGHT / 2.0
                {
                    velocity.velocity.y *= -1.0;
                }
            }
            ShapeType::Rectangle(width, height ) | ShapeType::Rhombus(width, height) => {
                if transform.translation.x + width > WINDOW_WIDTH / 2.0
                || transform.translation.x - width < -WINDOW_WIDTH / 2.0
                {
                    velocity.velocity.x *= -1.0;
                }
                if transform.translation.y + height > WINDOW_HEIGHT / 2.0
                || transform.translation.y - height < -WINDOW_HEIGHT / 2.0
                {
                    velocity.velocity.y *= -1.0;
                }
            }
            _ => {}
        }
    }
}
