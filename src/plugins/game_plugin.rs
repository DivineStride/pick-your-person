use bevy::prelude::*;
use crate::{
    resources::WaitingTimer,
    states::GameState,
    systems::{
        countdown::*, countdown_visuals::*, selection::*, victory::*, winner::*
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
                (update_countdown, animate_countdown_rings, animate_glow_points).run_if(
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
                (
                    (
                        select_winner,
                        setup_winner_timer
                    ).before(setup_victory_background),
                    setup_victory_background
                )
            )
            .add_systems(
                Update,
                (
                    animate_victory_background,
                    cleanup_non_winners,
                    (
                        highlight_winner,
                        check_winner_timer,
                    )
                        .before(animate_victory_background)
                        .before(cleanup_non_winners),
                ).run_if(in_state(GameState::WinnerChickenDinner))
            )
            .add_systems(
                OnExit(GameState::WinnerChickenDinner),
                (cleanup_winner, cleanup_victory_background),
            );
    }
}

