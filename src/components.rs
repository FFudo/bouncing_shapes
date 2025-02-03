use bevy::prelude::*;

#[derive(Resource)]
pub struct ImpulsTimer(pub Timer);

#[derive(Component)]
pub enum ShapeType {
    Circle(f32),          // radius
    Rectangle(f32, f32),  // width, height
    Annulus(f32, f32),    // inner radius, outer radius
    Rhombus(f32, f32),    // width, height
    RegularPolygon(f32, usize), // radius, sides
    Triangle(Vec2, Vec2, Vec2), // vertices
}

#[derive(Component)]
pub struct NeedsCollider;