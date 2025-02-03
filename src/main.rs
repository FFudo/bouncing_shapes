use bevy::{prelude::*, window::*};
use bevy_rapier2d::prelude::*;
use rand::*;

mod components;
mod spawn_shapes;

use crate::components::*;
use crate::spawn_shapes::SpawnShapesPlugin;

pub const WINDOW_WIDTH: f32 = 1080.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, apply_impuls)
        .run();
}

fn setup(mut query: Query<&mut RapierConfiguration>, mut commands: Commands) {
    for mut config in query.iter_mut() {
        config.gravity = Vec2::ZERO;
    }
    commands.spawn(Camera2d::default());
}

fn apply_impuls(
    time: Res<Time>,
    mut timer: ResMut<ImpulsTimer>,
    mut query: Query<&mut ExternalImpulse>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        for mut external_impulse in query.iter_mut() {
            if rng.gen_bool(0.4) {
                let dir_x = if rng.gen_bool(0.5) { 1 } else { -1 } as f32;
                let dir_y = if rng.gen_bool(0.5) { 1 } else { -1 } as f32;
                external_impulse.impulse = Vec2::new(
                    rng.gen_range(1..10) as f32 * dir_x * 100000.0,
                    rng.gen_range(1..10) as f32 * dir_y * 100000.0,
                );
            }
        }
    }
}