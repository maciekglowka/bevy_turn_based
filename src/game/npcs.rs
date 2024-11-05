use bevy::prelude::*;

use super::{
    actions::{get_action_at, Action},
    utils::spawn_sprite_bundle_at,
};
use crate::{
    assets::SpriteTextures,
    components::{Attack, Health, Npc, Obstacle, Position},
    globals::SPRITE_Z,
};

pub fn spawn_npcs(mut commands: Commands, textures: Res<SpriteTextures>) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    for y in 3..=4 {
        let entity = spawn_sprite_bundle_at(&mut commands, &atlas, 13, IVec2::new(4, y), SPRITE_Z);
        commands
            .entity(entity)
            .insert((Attack(1), Health(3), Npc, Obstacle));
    }
}

pub fn get_npc_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let position = world.get::<Position>(entity)?.0;
    for d in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
        if let Some(action) = get_action_at(entity, position + d, world) {
            return Some(action);
        }
    }
    None
}
