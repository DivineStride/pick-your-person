use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::components::Finger;

pub fn handle_keyboard_input(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    finger_query: Query<(Entity, &Finger)>,
) {
    let Ok(window) = window_query.single() else { return };
    let window_width = window.width();
    let window_height = window.height();

    // Pad the sides of the windows so that we don't get half circles
    let radius = 50.0;
    let additional_padding = 30.0;
    let padded_width = window_width - radius - additional_padding;
    let padded_height = window_height - radius - additional_padding;

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
            let x = rng.random_range(-padded_width/2.0..padded_width/2.0);
            let y = rng.random_range(-padded_height/2.0..padded_height/2.0);

            let color = Color::srgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            );
            
            let z_offset = time.elapsed_secs() * 0.01;
            commands.spawn((
                Finger {
                    key: *key,
                    color,
                },
                Mesh2d(meshes.add(Circle::new(radius))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x, y, z_offset)
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
