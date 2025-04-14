use bevy::prelude::*;

#[derive(Component)]
pub struct Finger {
    pub key: KeyCode,
    pub color: Color,
}

#[derive(Component)]
pub struct CountdownRing {
    pub radius: f32,
    pub progress: f32,
    pub complete: bool,
}

#[derive(Component)]
pub struct GlowPoint {
    pub angle: f32,
}

#[derive(Component)]
pub struct Winner;

#[derive(Component)]
pub struct VictoryBackground;


