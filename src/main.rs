use bevy::prelude::*;

mod states;
mod plugins;
mod systems;
mod components;
mod resources;

use plugins::{GameFlowPlugin, SettingsPlugin, InputPlugin};
use states::{GameState, AppState};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(SettingsPlugin)
        .add_plugins(GameFlowPlugin)
        .add_plugins(InputPlugin)
        .run();
}
