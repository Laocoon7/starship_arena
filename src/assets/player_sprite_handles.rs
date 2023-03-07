use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerSpriteHandles {
    pub handles: Vec<Handle<Image>>,

    pub ship_blue: Option<Handle<Image>>,
    pub ship_green: Option<Handle<Image>>,
    pub ship_grey: Option<Handle<Image>>,
    pub ship_orange: Option<Handle<Image>>,
    pub ship_red: Option<Handle<Image>>,
    pub ship_white: Option<Handle<Image>>,
}

impl PlayerSpriteHandles {
    pub fn load(
        &mut self,
        asset_server: &Res<AssetServer>,
        _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        trace!("Loading Player Sprites");

        let mut handles = Vec::new();

        let texture_handle = asset_server.load("ships/player/ShipA_Blue.png");
        handles.push(texture_handle.clone());
        self.ship_blue = Some(texture_handle);
        let texture_handle = asset_server.load("ships/player/ShipA_Green.png");
        handles.push(texture_handle.clone());
        self.ship_green = Some(texture_handle);
        let texture_handle = asset_server.load("ships/player/ShipA_Grey.png");
        handles.push(texture_handle.clone());
        self.ship_grey = Some(texture_handle);
        let texture_handle = asset_server.load("ships/player/ShipA_Orange.png");
        handles.push(texture_handle.clone());
        self.ship_orange = Some(texture_handle);
        let texture_handle = asset_server.load("ships/player/ShipA_Red.png");
        handles.push(texture_handle.clone());
        self.ship_red = Some(texture_handle);
        let texture_handle = asset_server.load("ships/player/ShipA_White.png");
        handles.push(texture_handle.clone());
        self.ship_white = Some(texture_handle);

        self.handles = handles;
    }
}
