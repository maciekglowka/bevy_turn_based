use bevy::prelude::*;

mod board;
mod player;
mod utils;

pub struct DungeonPlugin;
impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (board::spawn_board, player::spawn_player));
    }
}
