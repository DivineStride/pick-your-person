use bevy::prelude::*;

use crate::{
    components::{
        Finger,
        CountdownRing,
        GlowPoint,
    },
    resources::CountdownTimer,
};

pub fn setup_countdown_rings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    finger_query: Query<(Entity, &Finger, &Transform)>,
) {
    for (entity, finger, transform) in finger_query.iter() {
        let ring_radius = 55.0;

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                CountdownRing {
                    radius: ring_radius,
                    progress: 0.0,
                    complete: false,
                },
                Mesh2d(meshes.add(Annulus::new(ring_radius, ring_radius + 10.0))),
                MeshMaterial2d(materials.add(finger.color.with_alpha(0.8))),
                Transform::from_xyz(0.0, 0.0, -0.1),
            ));

            parent.spawn((
                GlowPoint { angle: 0.0 },
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(finger.color.with_luminance(0.8))),
                Transform::from_xyz(ring_radius + 5.0, 0.0, 0.0)
            ));
        });
    }
}

pub fn animate_countdown_rings(
    time: Res<Time>,
    countdown_timer: Res<CountdownTimer>,
) {
}

pub fn cleanup_countdown_visuals(
    mut commands: Commands,
    ring_query: Query<Entity, With<CountdownRing>>,
    glow_query: Query<Entity, With<GlowPoint>>,
) {
    for entity in ring_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in glow_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


