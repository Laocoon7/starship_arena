use bevy::prelude::*;

use crate::{assets::player_sprite_handles::PlayerSpriteHandles, states::AppState};

const TEMP_SPEED: f32 = 100.;

#[derive(Component)]
pub struct PlayerTag;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding PlayerPlugin systems");
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)));
        app.add_system(handle_input.in_set(OnUpdate(AppState::Game)));
    }
}

fn spawn_player(mut commands: Commands, player_sprite_handles: Res<PlayerSpriteHandles>) {
    info!("Spawning player");
    let texture = (player_sprite_handles.ship_blue)
        .clone()
        .expect("Error getting player sprite");
    commands.spawn((
        SpriteBundle {
            texture,
            ..Default::default()
        },
        PlayerTag,
    ));
}

fn handle_input(
    keyboard_input: Res<Input<ScanCode>>,
    mut player_query: Query<&mut Transform, With<PlayerTag>>,
    time: Res<Time>,
) {
    // Movement
    let mut direction = Vec2::ZERO;
    // W
    if keyboard_input.just_pressed(ScanCode(17)) || keyboard_input.pressed(ScanCode(17)) {
        direction.y += 1.;
    }
    // S
    if keyboard_input.just_pressed(ScanCode(31)) || keyboard_input.pressed(ScanCode(31)) {
        direction.y -= 1.;
    }
    // A
    if keyboard_input.just_pressed(ScanCode(30)) || keyboard_input.pressed(ScanCode(30)) {
        direction.x -= 1.;
    }
    // D
    if keyboard_input.just_pressed(ScanCode(32)) || keyboard_input.pressed(ScanCode(32)) {
        direction.x += 1.;
    }

    for mut transform in player_query.iter_mut() {
        transform.translation = Vec3 {
            x: transform.translation.x + direction.x * TEMP_SPEED * time.delta_seconds(),
            y: transform.translation.y + direction.y * TEMP_SPEED * time.delta_seconds(),
            z: transform.translation.z,
        };
    }
}
