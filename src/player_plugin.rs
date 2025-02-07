use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;

pub struct PlayerPlugin;

const ACCELERATION: f32 = 1500.0;
const DECELERATION: f32 = 1400.0;
const MAX_SPEED: f32 = 500.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let color = Color::hsl(240.0, 0.4, 0.8);
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
        Transform::from_xyz(-400.0, -400.0, 1.0),
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Ccd::enabled(),
        Friction::coefficient(0.5),
        NeedsCollider,
        Player,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = query.single_mut();
    let delta_time = time.delta_secs();

    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        velocity.linvel += direction * ACCELERATION * delta_time;
    } else {
        let deceleration = velocity.linvel.normalize() * DECELERATION * delta_time;
        if velocity.linvel.length() > deceleration.length() {
            velocity.linvel -= deceleration;
        } else {
            velocity.linvel = Vec2::ZERO;
        }
    }

    if velocity.linvel.length() > MAX_SPEED {
        velocity.linvel = velocity.linvel.normalize() * MAX_SPEED;
    }
}
