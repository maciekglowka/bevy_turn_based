use bevy::prelude::*;

use crate::{
    assets::Atlas,
    components::Position,
    globals::{SPRITE_SCALE, SPRITE_SIZE},
};

pub fn tile_to_world(v: IVec2, z: f32) -> Vec3 {
    SPRITE_SIZE * Vec3::new(v.x as f32, v.y as f32, z)
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
                transform: Transform::from_translation(tile_to_world(position, z))
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
