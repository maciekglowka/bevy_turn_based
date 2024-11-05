use bevy::prelude::*;

#[derive(Event)]
pub struct InputEvent(pub IVec2);

#[derive(Event)]
pub struct GameTick;

#[derive(Event)]
pub enum GameEvent {
    Move(Entity, IVec2),
    Attack(Entity, IVec2),
}
