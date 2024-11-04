use bevy::prelude::*;

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Player;
