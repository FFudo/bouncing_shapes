use bevy::prelude::*;

#[derive(Resource)]
pub struct ImpulsTimer(pub Timer);

#[derive(Component)]
pub struct NeedsCollider;