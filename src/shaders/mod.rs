use bevy::{prelude::*, sprite::Material2dPlugin};

pub mod countdown_shader;
pub mod victory_shader;

use countdown_shader::CountdownMaterial;
use victory_shader::VictoryMaterial;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
                Material2dPlugin::<CountdownMaterial>::default(),
                Material2dPlugin::<VictoryMaterial>::default(),
            )
        );
    }
}
