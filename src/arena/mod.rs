use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct WallTag;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding ArenaPlugin systems");
        app.add_startup_system(init_arena);
    }
}

fn init_arena(mut commands: Commands) {
    trace!("Spawning Arena");
    commands.spawn((
        Collider::cuboid(500., 50.),
        TransformBundle::from(Transform::from_xyz(0., -100., 0.)),
    ));
}
