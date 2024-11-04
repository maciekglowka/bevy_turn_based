use bevy::prelude::*;

use super::utils::{spawn_sprite_bundle_at, tile_to_world};
use crate::{
    assets::SpriteTextures,
    components::Position,
    globals::{BOARD_SIZE, BOARD_Z, SPRITE_SCALE},
};

pub fn spawn_board(mut commands: Commands, textures: Res<SpriteTextures>) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let v = IVec2::new(x as i32, y as i32);
            let _ = spawn_sprite_bundle_at(&mut commands, &atlas, 68, v, BOARD_Z);
        }
    }
}
