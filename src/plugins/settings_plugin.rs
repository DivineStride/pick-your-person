use bevy::prelude::*;

use crate::systems::app_control::handle_app_control;
use crate::states::AppState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(OnEnter(AppState::Settings), setup_settings_ui)
            .add_systems(Update, handle_app_control)
            .add_systems(Update, 
                (
                    handle_settings_input,
                    update_settings_display,
                ).run_if(in_state(AppState::Settings))
            )
            .add_systems(OnExit(AppState::Settings), cleanup_settings_ui);

    }
}

fn setup_settings_ui() {}

fn handle_settings_input() {}

fn update_settings_display() {}

fn cleanup_settings_ui() {}
