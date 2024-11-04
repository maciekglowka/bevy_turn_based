use actions::get_action_at;
use bevy::{
    ecs::{system::SystemId, world::Command},
    prelude::*,
};
use std::collections::VecDeque;

use crate::{
    components::{Actor, Player, Position},
    events::{GameTick, InputEvent},
};

mod actions;
mod animation;
mod board;
mod player;
mod utils;

// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
// enum TurnState {
//     #[default]
//     Player,
//     Npc,
// }
#[derive(Resource)]
struct QueueSystems {
    collect_actor_queue: SystemId,
    handle_actor_queue: SystemId,
}
impl FromWorld for QueueSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            collect_actor_queue: world.register_system(collect_actor_queue),
            handle_actor_queue: world.register_system(handle_actor_queue),
        }
    }
}

#[derive(Default, Resource)]
struct ActionQueue(VecDeque<Box<dyn actions::Action>>);

#[derive(Default, Resource)]
struct ActorQueue(VecDeque<Entity>);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionQueue>()
            .init_resource::<ActorQueue>()
            .init_resource::<QueueSystems>()
            .add_systems(PostStartup, (board::spawn_board, player::spawn_player))
            .add_systems(
                Update,
                (
                    handle_input_events,
                    animation::handle_game_events,
                    animation::handle_animations,
                ),
            )
            .add_systems(Update, handle_action_queue.run_if(on_event::<GameTick>()));
    }
}

fn handle_action_queue(world: &mut World) {
    if let Some(action) = world.resource_mut::<ActionQueue>().0.pop_front() {
        if action.is_valid(world) {
            let result = action.execute(world);
            if let Some(result) = result {
                world.resource_mut::<ActionQueue>().0.push_back(result);
            }
        }
    } else {
        let _ = world.run_system(world.resource::<QueueSystems>().handle_actor_queue);
    }
}

fn collect_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    player_query: Query<Entity, With<Player>>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0 = query.iter().collect();
    if let Ok(player) = player_query.get_single() {
        queue.0.push_front(player);
    }
}

fn handle_actor_queue(world: &mut World) {
    let Some(&entity) = world.resource::<ActorQueue>().0.front() else {
        let _ = world.run_system(world.resource::<QueueSystems>().collect_actor_queue);
        return;
    };

    // check if player has an intent buffered
    if let Some(mut player) = world.get_mut::<Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = get_action_at(entity, target, world) {
                world.resource_mut::<ActionQueue>().0.push_back(action);
                world.resource_mut::<ActorQueue>().0.pop_front();
            }
        }
        return;
    }
}

fn handle_input_events(
    mut events: EventReader<InputEvent>,
    mut query: Query<(&mut Player, &Position)>,
) {
    for event in events.read() {
        if let Ok((mut player, position)) = query.get_single_mut() {
            player.0 = Some(position.0 + event.0);
        }
    }
}

// struct AppendActionQueue(Box<dyn actions::Action>);
// impl Command for AppendActionQueue {
//     fn apply(self, world: &mut World) {
//         world.resource_mut::<ActionQueue>().0.push_back(self.0);
//     }
// }
