use bevy::prelude::*;

#[derive(Component)]
pub struct PhysicsObject2d {
    pub velocity: Vec2,
    pub drag: f32,
}

impl Default for PhysicsObject2d {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            drag: 1.0,
        }
    }
}

#[allow(unused)]
impl PhysicsObject2d {
    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}
