use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, check_player_input);
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
        Friction::coefficient(0.95),
        NeedsCollider,
        Player,
    ));
}

fn check_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            velocity.linvel.y += 5.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.linvel.y -= 5.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity.linvel.x += 5.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.linvel.x -= 5.0;
        }
        velocity.linvel = velocity.linvel.clamp(
            Vec2 {
                x: -200.0,
                y: -200.0,
            },
            Vec2 { x: 200.0, y: 200.0 },
        );
    }
}
