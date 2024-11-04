use bevy::prelude::*;

use crate::events::InputEvent;

pub fn handle_game_keyboard(
    mut input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<InputEvent>,
) {
    if input.just_released(KeyCode::KeyW) {
        input_events.send(InputEvent(IVec2::Y));
    }
    if input.just_released(KeyCode::KeyS) {
        input_events.send(InputEvent(IVec2::NEG_Y));
    }
    if input.just_released(KeyCode::KeyD) {
        input_events.send(InputEvent(IVec2::X));
    }
    if input.just_released(KeyCode::KeyA) {
        input_events.send(InputEvent(IVec2::NEG_X));
    }
}
