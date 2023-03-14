use bevy::{ecs::system::SystemParam, input::mouse::MouseMotion, prelude::*};

use crate::camera::GameCameraTag;

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding MousePlugin systems");
        app.init_resource::<MouseProperties>();
        app.add_system(update_mouse_properties.in_base_set(CoreSet::First));
    }
}

#[derive(SystemParam)]
pub struct Mouse<'w> {
    properties: Res<'w, MouseProperties>,
    // #[system_param(ignore)]
    // marker: PhantomData<Marker>,
}
#[allow(unused)]
impl<'w> Mouse<'w> {
    pub fn position(&self) -> Vec2 {
        self.properties.position
    }

    pub fn delta(&self) -> Vec2 {
        self.properties.delta
    }
}

#[derive(Resource)]
struct MouseProperties {
    position: Vec2,
    delta: Vec2,
}
impl Default for MouseProperties {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            delta: Vec2::ZERO,
        }
    }
}

fn update_mouse_properties(
    mut mouse: ResMut<MouseProperties>,

    mut position_events: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCameraTag>>,

    mut delta_events: EventReader<MouseMotion>,
) {
    // Update position
    // TODO: This needs improvement for window management
    for cursor_moved in position_events.iter() {
        let position = cursor_moved.position;
        for (current_camera, camera_transform) in camera_query.iter() {
            if let Some(world_position) =
                current_camera.viewport_to_world_2d(camera_transform, position)
            {
                mouse.position = world_position;
            }
        }
    }

    // Update delta
    let delta = delta_events
        .iter()
        .fold(Vec2::ZERO, |accumulator, event| accumulator + event.delta);
    mouse.delta = delta;
}
