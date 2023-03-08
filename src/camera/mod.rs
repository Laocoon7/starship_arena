use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

#[derive(Component)]
pub struct GameCameraTag;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding CameraPlugin systems");
        app.add_startup_system(init_game_camera);
    }
}

fn init_game_camera(mut commands: Commands) {
    trace!("Spawning Game Camera");
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for bloom
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::default(),
        GameCameraTag,
    ));
}
