use bevy::prelude::*;

use super::utils::spawn_sprite_bundle_at;
use crate::{
    assets::SpriteTextures,
    components::{Attack, Health, Obstacle, Player},
    globals::SPRITE_Z,
};

pub fn spawn_player(mut commands: Commands, textures: Res<SpriteTextures>) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    let entity = spawn_sprite_bundle_at(&mut commands, &atlas, 4, IVec2::ZERO, SPRITE_Z);
    commands
        .entity(entity)
        .insert((Attack(1), Health(3), Player::default(), Obstacle));
}
