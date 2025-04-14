use bevy::prelude::*;

use crate::resources::CountdownTimer;
use crate::states::GameState;

pub fn setup_countdown(mut commands: Commands) {
    commands.insert_resource(CountdownTimer::default());
}

pub fn update_countdown(
    time: Res<Time>,
    mut countdown: ResMut<CountdownTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    countdown.timer.tick(time.delta());

    if countdown.timer.finished() {
        next_state.set(GameState::WinnerChickenDinner);
    }
}

pub fn cleanup_countdown(mut commands: Commands) {
    commands.remove_resource::<CountdownTimer>();
}
