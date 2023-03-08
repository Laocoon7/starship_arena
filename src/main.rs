// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use assets::ai_sprite_handles::AiSpriteHandles;
use assets::player_sprite_handles::PlayerSpriteHandles;
use assets::projectile_sprite_handles::ProjectileSpriteHandles;
use bevy::app::{NoopPluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use states::AppState;

mod arena;
mod assets;
mod camera;
mod debug;
mod log;
mod physics;
mod player;
mod states;

fn main() {
    App::new()
        .init_resource::<PlayerSpriteHandles>()
        .init_resource::<AiSpriteHandles>()
        .init_resource::<ProjectileSpriteHandles>()
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(default_plugins())
        .add_plugins(my_plugins())
        .run();
}

fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                resolution: WindowResolution::new(1920., 1080.),
                title: "Starship Arena".to_string(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.,
                    min_height: 600.,
                    ..Default::default()
                },
                resizable: true,
                decorations: true,
                focused: true,
                canvas: None,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(log::my_log_plugin())
}

fn my_plugins() -> PluginGroupBuilder {
    let plugins = PluginGroupBuilder::start::<NoopPluginGroup>()
        .add(assets::AssetsPlugin)
        .add(camera::CameraPlugin)
        .add(physics::PhysicsPlugin {
            run_in_state: AppState::Game,
        })
        .add(player::PlayerPlugin);

    #[cfg(feature = "debug")]
    let plugins = plugins.add(debug::DebugPlugin);

    plugins
}

/*
//! Illustrates bloom post-processing in 2d.

use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_bloom_settings)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // Sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        sprite: Sprite {
            color: Color::rgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
            custom_size: Some(Vec2::splat(160.0)),
            ..default()
        },
        ..default()
    });

    // Circle mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(100.).into()).into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
    });

    // Hexagon mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
        ..default()
    });

    // UI
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/JuliaMono-Medium.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
    );
}

// ------------------------------------------------------------------------------------------------

fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
            text.push_str(&format!(
                "(W/S) Low-frequency boost: {}\n",
                bloom_settings.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frequency boost curvature: {}\n",
                bloom_settings.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency: {}\n",
                bloom_settings.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode: {}\n",
                match bloom_settings.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!(
                "(Y/H) Threshold: {}\n",
                bloom_settings.prefilter_settings.threshold
            ));
            text.push_str(&format!(
                "(U/J) Threshold softness: {}\n",
                bloom_settings.prefilter_settings.threshold_softness
            ));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::A) {
                bloom_settings.intensity -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::Q) {
                bloom_settings.intensity += dt / 10.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::S) {
                bloom_settings.low_frequency_boost -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::W) {
                bloom_settings.low_frequency_boost += dt / 10.0;
            }
            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::D) {
                bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::E) {
                bloom_settings.low_frequency_boost_curvature += dt / 10.0;
            }
            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::F) {
                bloom_settings.high_pass_frequency -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::R) {
                bloom_settings.high_pass_frequency += dt / 10.0;
            }
            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::G) {
                bloom_settings.composite_mode = BloomCompositeMode::Additive;
            }
            if keycode.pressed(KeyCode::T) {
                bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::H) {
                bloom_settings.prefilter_settings.threshold -= dt;
            }
            if keycode.pressed(KeyCode::Y) {
                bloom_settings.prefilter_settings.threshold += dt;
            }
            bloom_settings.prefilter_settings.threshold =
                bloom_settings.prefilter_settings.threshold.max(0.0);

            if keycode.pressed(KeyCode::J) {
                bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::U) {
                bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
            }
            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::default());
            }
        }
    }
}

//! Demonstrates handling a key press/release.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system)
        .run();
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}
*/
