use bevy::prelude::*;

use crate::states::AppState;

pub fn handle_app_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        next_app_state.set(AppState::InGame);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_app_state.set(AppState::Settings);
    }
}
