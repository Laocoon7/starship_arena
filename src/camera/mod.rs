use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::{player::PlayerTag, states::AppState};

#[derive(Component)]
pub struct GameCameraTag;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding CameraPlugin systems");
        app.add_startup_system(init_game_camera);
        app.add_system(center_camera_on_player.in_set(OnUpdate(AppState::Game)));
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

fn center_camera_on_player(
    mut camera_query: Query<&mut Transform, (With<GameCameraTag>, Without<PlayerTag>)>,
    player_query: Query<&Transform, (With<PlayerTag>, Without<GameCameraTag>)>,
) {
    for player_transform in player_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = Vec3 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
                z: camera_transform.translation.z,
            };
        }
    }
}
