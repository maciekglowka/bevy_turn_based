use bevy::prelude::*;

use crate::{
    assets::Atlas,
    components::Position,
    globals::{BOARD_SIZE, SPRITE_SCALE, SPRITE_SIZE},
};

pub fn tile_to_world(v: IVec2, z: Option<f32>) -> Vec3 {
    SPRITE_SIZE * Vec3::new(v.x as f32, v.y as f32, z.unwrap_or_default())
}

pub fn spawn_sprite_bundle_at<'a>(
    commands: &mut Commands,
    atlas: &Atlas,
    index: usize,
    position: IVec2,
    z: f32,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(tile_to_world(position, Some(z)))
                    .with_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.)),
                texture: atlas.texture.clone(),
                ..Default::default()
            },
            TextureAtlas {
                layout: atlas.layout.clone(),
                index,
            },
            Position(position),
        ))
        .id()
}

pub fn is_on_board(v: IVec2) -> bool {
    v.x >= 0 && v.y >= 0 && v.x < BOARD_SIZE as i32 && v.y < BOARD_SIZE as i32
}
