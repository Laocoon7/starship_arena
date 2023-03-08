use bevy::prelude::*;

use super::PhysicsObject2d;


#[derive(Bundle)]
pub struct PhysicsObjectBundle {
    pub name: Name,
    pub physics_object: PhysicsObject2d,
}

impl Default for PhysicsObjectBundle {
    fn default() -> Self {
        Self {
            name: Name::new("A Physics Object"),
            physics_object: PhysicsObject2d::default(),
        }
    }
}