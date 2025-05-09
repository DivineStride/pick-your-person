use bevy::prelude::*;

#[derive(Component)]
pub struct Finger {
    pub key: KeyCode,
    pub color: Color,
}

#[derive(Component)]
pub struct CountdownRing {
    pub progress: f32,
}

#[derive(Component)]
pub struct GlowPoint {
    pub angle: f32,
}

#[derive(Component)]
pub struct Winner;

#[derive(Component)]
pub struct VictoryBackground;


