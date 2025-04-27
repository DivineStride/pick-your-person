use bevy::prelude::*;

mod states;
mod plugins;
mod systems;
mod shaders;
mod components;
mod resources;

use plugins::{GameFlowPlugin, SettingsPlugin, InputPlugin};
use states::{GameState, AppState};
use shaders::ShaderPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(ShaderPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(GameFlowPlugin)
        .add_plugins(InputPlugin)
        .run();
}
