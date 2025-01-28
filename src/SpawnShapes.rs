use bevy::prelude::*;
use crate::components::*;

pub struct SpawnShapesPlugin;

impl Plugin for SpawnShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shapes);
    }
}

fn spawn_shapes(mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    rng: Res<RngResource>,) {
    commands.spawn(bundle)
}
