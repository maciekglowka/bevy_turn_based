use bevy::prelude::*;

mod actions;
mod assets;
mod components;
mod dungeon;
mod globals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(dungeon::DungeonPlugin)
        .insert_resource(assets::SpriteTextures::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, assets::load_assets)
        .run();
}

fn setup(mut commands: Commands) {
    let offset = 0.5 * globals::BOARD_SIZE as f32 * globals::SPRITE_SIZE;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(offset, offset, 0.)),
        ..Default::default()
    });
}
