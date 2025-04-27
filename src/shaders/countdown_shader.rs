use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

const COUNTDOWN_RING_PATH: &str = "shaders/countdown_ring.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CountdownMaterial {
    // The progress of the countdown (0.0 to 1.0)
    #[uniform(0)]
    pub progress: f32,

    // The base color of the ring
    #[uniform(1)]
    pub color: LinearRgba,

    // The secondary color for gradient effects
    #[uniform(2)]
    pub secondary_color: LinearRgba,
    
    // Radius of the ring
    #[uniform(3)]
    pub radius: f32,
    
    // Add a texture if needed
    #[texture(4)]
    #[sampler(5)]
    pub texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode2d,
}

impl Material2d for CountdownMaterial {
    fn fragment_shader() -> ShaderRef {
        COUNTDOWN_RING_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}
