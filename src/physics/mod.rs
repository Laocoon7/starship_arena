use bevy::prelude::*;

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

pub struct PhysicsPlugin<T> {
    pub run_in_state: T,
}

impl<T> Plugin for PhysicsPlugin<T>
where
    T: States + Copy,
{
    fn build(&self, app: &mut App) {
        trace!("Adding PhysicsPlugin systems");
        app.add_system(apply_velocities.in_set(OnUpdate(self.run_in_state)));
    }
}

fn apply_velocities(
    mut query_physics_objects: Query<(&mut PhysicsObject2d, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut physics_object, mut transform) in query_physics_objects.iter_mut() {
        transform.translation = Vec3::new(
            transform.translation.x + physics_object.velocity.x * time.delta_seconds(),
            transform.translation.y + physics_object.velocity.y * time.delta_seconds(),
            transform.translation.z,
        );

        physics_object.velocity.x -=
            physics_object.velocity.x * time.delta_seconds() * physics_object.drag;
        physics_object.velocity.y -=
            physics_object.velocity.y * time.delta_seconds() * physics_object.drag;
    }
}
