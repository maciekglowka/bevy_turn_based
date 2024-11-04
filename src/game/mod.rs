use bevy::{ecs::world::Command, prelude::*};
use std::collections::VecDeque;

use crate::{
    components::{Player, Position},
    events::{GameTick, InputEvent},
};

mod actions;
mod animation;
mod board;
mod player;
mod utils;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
enum TurnState {
    #[default]
    Player,
    Npc,
}

#[derive(Default, Resource)]
struct ActionQueue(VecDeque<Box<dyn actions::Action>>);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TurnState>()
            .init_resource::<ActionQueue>()
            .add_systems(PostStartup, (board::spawn_board, player::spawn_player))
            .add_systems(
                Update,
                (animation::handle_game_events, animation::handle_animations),
            )
            .add_systems(Update, handle_action_queue.run_if(on_event::<GameTick>()))
            .add_systems(
                Update,
                handle_input_events.run_if(in_state(TurnState::Player)),
            );
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
    };
}

fn handle_input_events(
    mut commands: Commands,
    mut events: EventReader<InputEvent>,
    query: Query<(Entity, &Position), With<Player>>,
) {
    for event in events.read() {
        if let Ok((player, position)) = query.get_single() {
            commands.add(AppendActionQueue(Box::new(actions::MoveAction {
                entity: player,
                target: position.0 + event.0,
            })));
        }
        break;
    }
}

struct AppendActionQueue(Box<dyn actions::Action>);
impl Command for AppendActionQueue {
    fn apply(self, world: &mut World) {
        world.resource_mut::<ActionQueue>().0.push_back(self.0);
    }
}
