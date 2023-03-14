use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::arena_sprite_handles::ArenaSpriteHandles, states::AppState};

#[derive(Component)]
pub struct WallTag;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding ArenaPlugin systems");
        app.add_system(init_arena.in_schedule(OnEnter(AppState::Game)));
    }
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}
impl Side {
    fn get_texture(&self, arena_sprite_handles: &Res<ArenaSpriteHandles>) -> Handle<Image> {
        match self {
            Side::Top => arena_sprite_handles
                .wall_left_right
                .clone()
                .expect("Error getting arena sprite"),
            Side::Bottom => arena_sprite_handles
                .wall_left_right
                .clone()
                .expect("Error getting arena sprite"),
            Side::Left => arena_sprite_handles
                .wall_top_bottom
                .clone()
                .expect("Error getting arena sprite"),
            Side::Right => arena_sprite_handles
                .wall_top_bottom
                .clone()
                .expect("Error getting arena sprite"),
        }
    }

    fn get_size(&self) -> Vec2 {
        match self {
            Side::Top => Vec2 { x: 1000., y: 100. },
            Side::Bottom => Vec2 { x: 1000., y: 100. },
            Side::Left => Vec2 { x: 100., y: 1000. },
            Side::Right => Vec2 { x: 100., y: 1000. },
        }
    }

    fn get_collider_size(&self) -> Vec2 {
        match self {
            Side::Top => Vec2 { x: 500., y: 25. },
            Side::Bottom => Vec2 { x: 500., y: 25. },
            Side::Left => Vec2 { x: 25., y: 500. },
            Side::Right => Vec2 { x: 25., y: 500. },
        }
    }

    fn get_position(&self) -> Vec2 {
        match self {
            Side::Top => Vec2 { x: 0., y: 500. },
            Side::Bottom => Vec2 { x: 0., y: -500. },
            Side::Left => Vec2 { x: -500., y: 0. },
            Side::Right => Vec2 { x: 500., y: 0. },
        }
    }
}

fn spawn_wall(
    commands: &mut Commands,
    arena_sprite_handles: &Res<ArenaSpriteHandles>,
    room: (u32, u32),
    side: Side,
    color: Color,
) -> Entity {
    let size = side.get_collider_size();
    let position = side.get_position();

    commands
        .spawn((
            SpriteBundle {
                texture: side.get_texture(arena_sprite_handles),
                sprite: Sprite {
                    color,
                    // flip_x
                    // flip_y
                    custom_size: Some(side.get_size()),
                    // rect
                    // anchor
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (room.0 * 1000) as f32 + position.x,
                    (room.1 * 1000) as f32 + position.y,
                    0.,
                ),
                // global_transform
                // visibility
                // computed_visibility
                ..Default::default()
            },
            Collider::cuboid(size.x, size.y),
        ))
        .id()
}

struct RoomDefinition {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

fn spawn_room(
    commands: &mut Commands,
    arena_sprite_handles: &Res<ArenaSpriteHandles>,
    room: (u32, u32),
    room_definition: RoomDefinition,
    color: Color,
) {
    if room_definition.top {
        spawn_wall(commands, arena_sprite_handles, room, Side::Top, color);
    }
    if room_definition.bottom {
        spawn_wall(commands, arena_sprite_handles, room, Side::Bottom, color);
    }
    if room_definition.left {
        spawn_wall(commands, arena_sprite_handles, room, Side::Left, color);
    }
    if room_definition.right {
        spawn_wall(commands, arena_sprite_handles, room, Side::Right, color);
    }
}

fn init_arena(mut commands: Commands, arena_sprite_handles: Res<ArenaSpriteHandles>) {
    trace!("Spawning Arena");

    spawn_room(
        &mut commands,
        &arena_sprite_handles,
        (0, 0),
        RoomDefinition {
            top: false,
            bottom: true,
            left: true,
            right: false,
        },
        Color::rgb(0., 5., 0.),
    );
    spawn_room(
        &mut commands,
        &arena_sprite_handles,
        (0, 1),
        RoomDefinition {
            top: true,
            bottom: false,
            left: true,
            right: true,
        },
        Color::rgb(0., 0., 5.),
    );
    spawn_room(
        &mut commands,
        &arena_sprite_handles,
        (1, 0),
        RoomDefinition {
            top: true,
            bottom: true,
            left: false,
            right: true,
        },
        Color::rgb(5., 0., 0.),
    );
}
