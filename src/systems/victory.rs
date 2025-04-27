use bevy::{prelude::*, window::PrimaryWindow, sprite::AlphaMode2d};
use crate::{
    components::{Finger, VictoryBackground, Winner}, 
    shaders::victory_shader::VictoryMaterial
};

pub fn setup_victory_background(
    mut commands: Commands,
    _time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    winner_query: Query<(&Finger, &Transform), With<Winner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut victory_materials: ResMut<Assets<VictoryMaterial>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    
    if let Ok((winner_finger, winner_transform)) = winner_query.single() {
        let window_size = Vec2::new(window.width(), window.height());
        let screen_position = winner_transform.translation.truncate() + window_size / 2.0;
        let normalized_position = screen_position / window_size;
        let aspect_ratio = window_size.x / window_size.y;
        
        let material = victory_materials.add(VictoryMaterial {
            color: winner_finger.color.to_linear(),
            time: 0.0,
            center: vec2(normalized_position.x * aspect_ratio, 1.0 - normalized_position.y),
            ripple_speed: 6.0,
            ripple_intensity: 0.7,
            aspect_ratio,
            alpha_mode: AlphaMode2d::Blend,
        });

        commands.spawn((
            VictoryBackground,
            Mesh2d(meshes.add(Rectangle::from_size(window_size))),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, -10.0)
        ));
    }

    
}

pub fn animate_victory_background(
    time: Res<Time>,
    mut query: Query<&MeshMaterial2d<VictoryMaterial>, With<VictoryBackground>>,
    mut materials: ResMut<Assets<VictoryMaterial>>,
) {
    for material_handle in query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.time += time.delta_secs();

            material.ripple_intensity = (material.time * 0.3).min(1.0);

            material.ripple_speed = 1.0 + (time.elapsed_secs() * 0.5).sin() * 0.3;
        }
    }
}

pub fn cleanup_victory_background(
    mut commands: Commands,
    mut query: Query<Entity, With<VictoryBackground>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
