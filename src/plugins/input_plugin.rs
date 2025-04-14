use bevy::prelude::*;

use crate::states::{AppState, GameState};
use crate::systems::input::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
            Update,
            handle_keyboard_input
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Waiting)
                        .or(in_state(GameState::Initiated))
                )
            )
            .add_systems(OnExit(GameState::WinnerChickenDinner), clean_input_entities);
    }
}
