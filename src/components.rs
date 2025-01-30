use bevy::prelude::*;

#[derive(Resource)]
pub struct ImpulsTimer(pub Timer);

#[derive(Component)]
pub struct Impuls;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
    pub friction: f32,
}

#[derive(Component)]
pub struct RectangleShape {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct CircleShape {
    pub radius: f32,
}

#[derive(Component)]
pub struct TriangleShape {
    pub points: [(f32, f32); 3],
}

#[derive(Component)]
pub struct RegularPolygonShape {
    pub circumradius: f32,
    pub sides: f32,
}

#[derive(Component)]
pub struct RhombusShape {
    pub diagonal1: f32,
    pub diagonal2: f32,
}

