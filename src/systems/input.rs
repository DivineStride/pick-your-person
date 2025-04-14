use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::components::Finger;

pub fn handle_keyboard_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    finger_query: Query<(Entity, &Finger)>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    let existing_keys: Vec<KeyCode> = finger_query
        .iter()
        .map(|(_, finger)| finger.key)
        .collect();

    let keys_to_check = [
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
        KeyCode::KeyF,
        KeyCode::KeyJ,
        KeyCode::KeyK,
        KeyCode::KeyL,
        KeyCode::Semicolon,
    ];

    for key in keys_to_check.iter() {
        if keyboard_input.just_pressed(*key) && !existing_keys.contains(key) {
            let mut rng = rand::rng();
            let x = rng.random_range(-window_width/2.0..window_width/2.0);
            let y = rng.random_range(-window_height/2.0..window_height/2.0);

            let color = Color::srgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            );
            
            commands.spawn((
                Finger {
                    key: *key,
                    color,
                },
                Mesh2d(meshes.add(Circle::new(50.0))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x, y, 0.0)
            ));
        }
    }

    for (entity, finger) in finger_query.iter() {
        if keyboard_input.just_released(finger.key) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn clean_input_entities(mut commands: Commands, finger_query: Query<(Entity, &Finger)>) {
    for (entity, _finger) in finger_query.iter() {
        commands.entity(entity).despawn();
    }
}
