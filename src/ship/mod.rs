use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_sphere_ship(
    commands: &mut Commands,
    position: Vec2,
    texture: &Handle<Image>,
    color: Color,
    radius: f32,
    mass: f32,
) -> Entity {
    let diameter = radius * 2.;
    commands
        .spawn((
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    color,
                    // flip_x
                    // flip_y
                    custom_size: Some(Vec2 {
                        x: diameter,
                        y: diameter,
                    }),
                    // rect
                    // anchor
                    ..Default::default()
                },
                transform: Transform::from_xyz(position.x, position.y, 0.),
                // global_transform
                // visibility
                // computed_visibility
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(radius),
            Restitution::coefficient(0.7),
            AdditionalMassProperties::Mass(mass),
            GravityScale(0.),
            Velocity::zero(),
        ))
        .id()
}
