use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, AlphaMode2d},
};

const VICTORY_PATH: &str = "shaders/victory.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VictoryMaterial {
    #[uniform(0)]
    pub color: LinearRgba,

    #[uniform(1)]
    pub time: f32,

    #[uniform(2)]
    pub center: Vec2,

    #[uniform(3)]
    pub ripple_speed: f32,

    #[uniform(4)]
    pub ripple_intensity: f32,
    
    #[uniform(5)]
    pub aspect_ratio: f32,
    pub alpha_mode: AlphaMode2d,
}

impl Material2d for VictoryMaterial {
    fn fragment_shader() -> ShaderRef {
        VICTORY_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}
