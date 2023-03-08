use bevy::prelude::*;

pub mod physics_object_bundle;
pub use physics_object_bundle::*;
pub mod physics_object_2d;
pub use physics_object_2d::*;

#[derive(Component)]
pub struct Collider;

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
