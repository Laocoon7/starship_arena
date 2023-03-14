use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    assets::player_sprite_handles::PlayerSpriteHandles,
    ship::{spawn_sphere_ship, ShipSpeed},
    states::AppState,
};

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

    let ship = spawn_sphere_ship(
        &mut commands,
        Vec2 { x: 0., y: 5. },
        &texture,
        Color::rgb(0., 0., 2.),
        50.,
        100.,
        ShipSpeed(25.),
    );
    commands.entity(ship).insert(PlayerTag);

    // TODO: Remove these for fun ships
    spawn_sphere_ship(
        &mut commands,
        Vec2 { x: 50., y: 10. },
        &texture,
        Color::rgb(0., 2., 0.),
        50.,
        10.,
        ShipSpeed(25.),
    );

    spawn_sphere_ship(
        &mut commands,
        Vec2 { x: 50., y: 10. },
        &texture,
        Color::rgb(2., 0., 0.),
        50.,
        100.,
        ShipSpeed(25.),
    );

    spawn_sphere_ship(
        &mut commands,
        Vec2 { x: 50., y: 10. },
        &texture,
        Color::rgb(2., 2., 0.),
        50.,
        1000.,
        ShipSpeed(25.),
    );
}

fn handle_input(
    keyboard_input: Res<Input<ScanCode>>,
    mut player_query: Query<(&mut Velocity, &ShipSpeed), With<PlayerTag>>,
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

    for (mut player_velocity, ship_speed) in player_query.iter_mut() {
        player_velocity.linvel += Vec2 {
            x: direction.x,
            y: direction.y,
        } * ship_speed.0;
        // physics_object.add_velocity(Vec2::new(direction.x, direction.y) * TEMP_SPEED);
    }
}
