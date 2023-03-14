use bevy::{asset::LoadState, prelude::*};

use crate::states::AppState;

use self::{
    ai_sprite_handles::AiSpriteHandles, arena_sprite_handles::ArenaSpriteHandles,
    player_sprite_handles::PlayerSpriteHandles, projectile_sprite_handles::ProjectileSpriteHandles,
};

pub mod ai_sprite_handles;
pub mod arena_sprite_handles;
pub mod player_sprite_handles;
pub mod projectile_sprite_handles;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding AssetPlugin systems");
        app.add_system(init_assets.in_schedule(OnEnter(AppState::Setup)));
        app.add_system(check_assets.in_set(OnUpdate(AppState::Setup)));
    }
}

pub fn init_assets(
    mut player_sprite_handles: ResMut<PlayerSpriteHandles>,
    mut ai_sprite_handles: ResMut<AiSpriteHandles>,
    mut arena_sprite_handles: ResMut<ArenaSpriteHandles>,
    mut projectile_sprite_handles: ResMut<ProjectileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    trace!("Initializing assets");
    player_sprite_handles.load(&asset_server, &mut texture_atlases);
    ai_sprite_handles.load(&asset_server, &mut texture_atlases);
    arena_sprite_handles.load(&asset_server, &mut texture_atlases);
    projectile_sprite_handles.load(&asset_server, &mut texture_atlases);
}

pub fn check_assets(
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    ai_sprite_handles: ResMut<AiSpriteHandles>,
    arena_sprite_handles: ResMut<ArenaSpriteHandles>,
    projectile_sprite_handles: ResMut<ProjectileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Checking assets");
    if let LoadState::Loaded = asset_server.get_group_load_state(
        player_sprite_handles
            .handles
            .iter()
            .map(|handle| handle.id()),
    ) {
        trace!("Player Loaded");
        if let LoadState::Loaded = asset_server
            .get_group_load_state(ai_sprite_handles.handles.iter().map(|handle| handle.id()))
        {
            trace!("Ai Loaded");
            if let LoadState::Loaded = asset_server.get_group_load_state(
                projectile_sprite_handles
                    .handles
                    .iter()
                    .map(|handle| handle.id()),
            ) {
                trace!("Projectiles Loaded");
                if let LoadState::Loaded = asset_server.get_group_load_state(
                    arena_sprite_handles
                        .handles
                        .iter()
                        .map(|handle| handle.id()),
                ) {
                    trace!("Arena Loaded");
                    next_state.set(AppState::Game);
                }
            }
        }
    }
}
