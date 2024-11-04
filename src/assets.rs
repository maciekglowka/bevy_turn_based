use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct SpriteTextures(pub HashMap<&'static str, Atlas>);

pub struct Atlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<SpriteTextures>,
) {
    let texture = server.load("colored_tilemap.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
    let layout_handle = atlas_layouts.add(layout);
    textures.0.insert(
        "TileMap",
        Atlas {
            layout: layout_handle,
            texture,
        },
    );
}
