use bevy::prelude::*;

use crate::{
    components::Finger,
    resources::WaitingTimer,
    states::GameState
};

pub fn setup_initiated(mut commands: Commands) {
    commands.insert_resource(WaitingTimer::default());
}

pub fn check_waiting(
    time: Res<Time>,
    mut waiting: ResMut<WaitingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    waiting.timer.tick(time.delta());

    if waiting.timer.finished() {
        next_state.set(GameState::Countdown);
    }
}

pub fn clear_waiting_timer(mut commands: Commands) {
    commands.remove_resource::<WaitingTimer>();
}

pub fn check_finger_count(
    finger_query: Query<&Finger>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let finger_count = finger_query.iter().count();

    match game_state.get() {
        GameState::Waiting => {
            if finger_count >= 2 {
                next_game_state.set(GameState::Initiated);
            }
        },
        GameState::Initiated | GameState::Countdown => {
            if finger_count < 2 {
                next_game_state.set(GameState::Waiting);
            }
        },
        _ => {}
    }
}



