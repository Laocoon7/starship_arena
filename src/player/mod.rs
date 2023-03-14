use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::player_sprite_handles::PlayerSpriteHandles, states::AppState};

const TEMP_SPEED: f32 = 25.;

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
                // flip_x
                // flip_y
                custom_size: Some(Vec2 { x: 100., y: 100. }),
                // rect
                // anchor
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 5., 0.),
            // global_transform
            // visibility
            // computed_visibility
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.),
        Restitution::coefficient(0.7),
        Velocity::zero(),
        PlayerTag,
    ));
}

fn handle_input(
    keyboard_input: Res<Input<ScanCode>>,
    mut player_query: Query<&mut Velocity, With<PlayerTag>>,
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

    for mut player_velocity in player_query.iter_mut() {
        player_velocity.linvel += Vec2 {
            x: direction.x,
            y: direction.y,
        } * TEMP_SPEED;
        // physics_object.add_velocity(Vec2::new(direction.x, direction.y) * TEMP_SPEED);
    }
}
