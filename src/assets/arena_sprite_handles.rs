use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ArenaSpriteHandles {
    pub handles: Vec<Handle<Image>>,

    pub wall_bottom_left: Option<Handle<Image>>,
    pub wall_bottom_right: Option<Handle<Image>>,
    pub wall_left_right: Option<Handle<Image>>,
    pub wall_top_bottom: Option<Handle<Image>>,
    pub wall_top_left: Option<Handle<Image>>,
    pub wall_top_right: Option<Handle<Image>>,
}

impl ArenaSpriteHandles {
    pub fn load(
        &mut self,
        asset_server: &Res<AssetServer>,
        _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        trace!("Loading Arena Sprites");

        let mut handles = Vec::new();

        let texture_handle = asset_server.load("arena/walls/bottom_left.png");
        handles.push(texture_handle.clone());
        self.wall_bottom_left = Some(texture_handle);

        let texture_handle = asset_server.load("arena/walls/bottom_right.png");
        handles.push(texture_handle.clone());
        self.wall_bottom_right = Some(texture_handle);

        let texture_handle = asset_server.load("arena/walls/left_right.png");
        handles.push(texture_handle.clone());
        self.wall_left_right = Some(texture_handle);

        let texture_handle = asset_server.load("arena/walls/top_bottom.png");
        handles.push(texture_handle.clone());
        self.wall_top_bottom = Some(texture_handle);

        let texture_handle = asset_server.load("arena/walls/top_left.png");
        handles.push(texture_handle.clone());
        self.wall_top_left = Some(texture_handle);

        let texture_handle = asset_server.load("arena/walls/top_right.png");
        handles.push(texture_handle.clone());
        self.wall_top_right = Some(texture_handle);

        self.handles = handles;
    }
}
