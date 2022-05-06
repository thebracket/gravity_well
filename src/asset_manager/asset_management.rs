use bevy::prelude::*;

pub struct AssetManager {
    pub menu_atlas: Handle<TextureAtlas>,
    pub default_font: Handle<Font>,
    pub atlases: Vec<Handle<TextureAtlas>>,
}
