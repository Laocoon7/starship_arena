use bevy::prelude::*;

use crate::{
    assets::player_sprite_handles::PlayerSpriteHandles,
    physics::{PhysicsObject2d, PhysicsObjectBundle},
    states::AppState,
};

const TEMP_SPEED: f32 = 50.;

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
    trace!("Spawning player");
    let texture = (player_sprite_handles.ship_blue)
        .clone()
        .expect("Error getting player sprite");
    commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite {
                color: Color::rgb(0., 0., 2.),
                ..Default::default()
            },
            ..Default::default()
        },
        PhysicsObjectBundle {
            name: Name::new("Player"),
            physics_object: PhysicsObject2d {
                drag: 3.5,
                ..Default::default()
            },
        },
        PlayerTag,
    ));
}

fn handle_input(
    keyboard_input: Res<Input<ScanCode>>,
    mut player_query: Query<&mut PhysicsObject2d, With<PlayerTag>>,
    //time: Res<Time>,
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

    for mut physics_object in player_query.iter_mut() {
        physics_object.add_velocity(Vec2::new(direction.x, direction.y) * TEMP_SPEED);
    }
}
