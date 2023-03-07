use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ProjectileSpriteHandles {
    pub handles: Vec<Handle<Image>>,

    pub basic_projectile: Option<Handle<Image>>,
}

impl ProjectileSpriteHandles {
    pub fn load(
        &mut self,
        asset_server: &Res<AssetServer>,
        _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        trace!("Loading Projectile Sprites");

        let mut handles = Vec::new();

        let texture_handle = asset_server.load("projectiles/8x16_BasicProjectile_White.png");
        handles.push(texture_handle.clone());
        self.basic_projectile = Some(texture_handle);

        self.handles = handles;
    }
}
