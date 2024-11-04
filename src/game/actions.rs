use bevy::prelude::*;

use super::utils::is_on_board;
use crate::{
    components::{Obstacle, Position},
    events::GameEvent,
};

pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    let actions: Vec<Box<dyn Action>> = vec![Box::new(MoveAction { entity, target })];
    for action in actions {
        if action.is_valid(world) {
            return Some(action);
        }
    }
    None
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
    fn is_valid(&self, world: &mut World) -> bool {
        true
    }
}

pub struct MoveAction {
    pub entity: Entity,
    pub target: IVec2,
}
impl Action for MoveAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        world.get_mut::<Position>(self.entity)?.0 = self.target;
        world.send_event::<GameEvent>(GameEvent::Move(self.entity, self.target));
        None
    }
    fn is_valid(&self, world: &mut World) -> bool {
        if !is_on_board(self.target) {
            return false;
        }
        if !get_entities_with::<Obstacle>(self.target, world).is_empty() {
            return false;
        }
        true
    }
}

fn get_entities_with<T: Component>(v: IVec2, world: &mut World) -> Vec<Entity> {
    world
        .query_filtered::<(Entity, &Position), With<T>>()
        .iter(&world)
        .filter(|(_, p)| p.0 == v)
        .map(|(e, _)| e)
        .collect()
}
