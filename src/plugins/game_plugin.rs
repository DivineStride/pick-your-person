use bevy::prelude::*;
use crate::{
    resources::WaitingTimer,
    states::GameState,
    systems::{
        countdown::*,
        countdown_visuals::*,
        selection::*,
        winner::*
    }
};

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app
            // --------------- Getting Inputs -------------------
            .add_systems(
                Update,
                check_finger_count.run_if(
                    in_state(GameState::Waiting)
                        .or(in_state(GameState::Initiated))
                        .or(in_state(GameState::Countdown))
                )
            )

            // --------------- Finding Initial Players ----------
            // We need to have a delay before we start the countdown
            .add_systems(OnEnter(GameState::Initiated), setup_initiated)
            .add_systems(Update, check_waiting.run_if(
                in_state(GameState::Initiated)
            ))
            // Reset waiting timer if we don't have enough players 
            .add_systems(OnEnter(GameState::Waiting), clear_waiting_timer.run_if(
                resource_exists::<WaitingTimer>
            ))

            // --------------- Beginning Countdown ----------------
            .add_systems(
                OnEnter(GameState::Countdown),
                (setup_countdown, setup_countdown_rings)
            )
            .add_systems(
                Update,
                (update_countdown, animate_countdown_rings).run_if(
                    in_state(GameState::Countdown)
                )
            )
            .add_systems(
                OnExit(GameState::Countdown),
                (cleanup_countdown, cleanup_countdown_visuals)
            )

            // --------------- Finding Winner --------------------
            .add_systems(
                OnEnter(GameState::WinnerChickenDinner), 
                (select_winner, setup_winner_timer)
            )
            .add_systems(
                Update,
                (
                    highlight_winner,
                    check_winner_timer
                ).run_if(in_state(GameState::WinnerChickenDinner))
            )
            .add_systems(
                OnExit(GameState::WinnerChickenDinner),
                cleanup_winner
            );
    }
}

