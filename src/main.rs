use bevy::{color::palettes::css::RED, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)] struct Player;

fn setup(mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((Player, Mesh2d(meshes.add(Circle::new(50.0))), MeshMaterial2d(materials.add(ColorMaterial::from_color(RED)))));
}

