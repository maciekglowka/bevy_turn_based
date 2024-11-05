use bevy::prelude::*;

use super::utils::is_on_board;
use crate::{
    components::{Attack, Health, Npc, Obstacle, Position},
    events::GameEvent,
};

pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    let actions: Vec<Box<dyn Action>> = vec![
        Box::new(MoveAction { entity, target }),
        Box::new(AttackAction { entity, target }),
    ];
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

pub struct AttackAction {
    pub entity: Entity,
    pub target: IVec2,
}
impl Action for AttackAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        // currently won't handle two target entities on the same tile!
        let &target = get_entities_with::<Health>(self.target, world).first()?;
        let damage = world.get::<Attack>(self.entity)?.0;
        world.send_event::<GameEvent>(GameEvent::Attack(self.entity, self.target));
        Some(Box::new(DamageAction {
            entity: target,
            value: damage,
        }))
    }
    fn is_valid(&self, world: &mut World) -> bool {
        if !is_on_board(self.target) {
            return false;
        }
        let mut targets = get_entities_with::<Health>(self.target, world);
        if targets.is_empty() {
            return false;
        }

        // disallow friendly fire
        targets.push(self.entity);
        let npcs = targets
            .iter()
            .filter(|&&e| world.get::<Npc>(e).is_some())
            .count();
        npcs == 1
    }
}

pub struct DamageAction {
    pub entity: Entity,
    pub value: u32,
}
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        let mut health = world.get_mut::<Health>(self.entity)?;
        health.0 = health.0.saturating_sub(self.value);
        if health.0 == 0 {
            return Some(Box::new(KillAction {
                entity: self.entity,
            }));
        }
        None
    }
}

pub struct KillAction {
    pub entity: Entity,
}
impl Action for KillAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        world.despawn(self.entity);
        None
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
