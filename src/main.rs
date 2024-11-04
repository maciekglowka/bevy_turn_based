use bevy::prelude::*;

mod assets;
mod components;
mod events;
mod game;
mod globals;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(game::GamePlugin)
        .insert_resource(assets::SpriteTextures::default())
        .add_event::<events::InputEvent>()
        .add_event::<events::GameEvent>()
        .add_event::<events::GameTick>()
        .add_systems(Startup, (setup, assets::load_assets))
        .add_systems(Update, input::handle_game_keyboard)
        .run();
}

fn setup(mut commands: Commands) {
    let offset = 0.5 * globals::BOARD_SIZE as f32 * globals::SPRITE_SIZE;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(offset, offset, 0.)),
        ..Default::default()
    });
}
