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

fn init_arena(mut commands: Commands, arena_sprite_handles: Res<ArenaSpriteHandles>) {
    trace!("Spawning Arena");
    let texture = (arena_sprite_handles.wall_left_right)
        .clone()
        .expect("Error getting arena sprites");
    // Top
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 5., 0.),
                // flip_x
                // flip_y
                custom_size: Some(Vec2 { x: 1600., y: 100. }),
                // rect
                // anchor
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 500., 0.),
            // global_transform
            // visibility
            // computed_visibility
            ..Default::default()
        },
        Collider::cuboid(800., 25.),
    ));
    // Bottom
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 5., 0.),
                // flip_x
                // flip_y
                custom_size: Some(Vec2 { x: 1600., y: 100. }),
                // rect
                // anchor
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -500., 0.),
            // global_transform
            // visibility
            // computed_visibility
            ..Default::default()
        },
        Collider::cuboid(800., 25.),
    ));

    let texture = (arena_sprite_handles.wall_top_bottom)
        .clone()
        .expect("Error getting arena sprites");

    // Left
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 5., 0.),
                // flip_x
                // flip_y
                custom_size: Some(Vec2 { x: 100., y: 1000. }),
                // rect
                // anchor
                ..Default::default()
            },
            transform: Transform::from_xyz(-800., 0., 0.),
            // global_transform
            // visibility
            // computed_visibility
            ..Default::default()
        },
        Collider::cuboid(25., 500.),
    ));
    // Right
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 5., 0.),
                // flip_x
                // flip_y
                custom_size: Some(Vec2 { x: 100., y: 1000. }),
                // rect
                // anchor
                ..Default::default()
            },
            transform: Transform::from_xyz(800., 0., 0.),
            // global_transform
            // visibility
            // computed_visibility
            ..Default::default()
        },
        Collider::cuboid(25., 500.),
    ));
}
