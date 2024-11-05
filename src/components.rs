use bevy::prelude::*;
use std::collections::VecDeque;

pub enum AnimationKind {
    Translate(VecDeque<Vec3>),
}

#[derive(Component)]
pub struct Animation(pub AnimationKind);

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component, Default)]
pub struct Player(pub Option<IVec2>);
