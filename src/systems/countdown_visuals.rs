use bevy::{prelude::*, sprite::AlphaMode2d};
use rand::Rng;

use crate::{
    components::{
        CountdownRing, Finger, GlowPoint
    },
    resources::CountdownTimer,
    shaders::countdown_shader::CountdownMaterial,
};

pub fn setup_countdown_rings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ring_material: ResMut<Assets<CountdownMaterial>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    finger_query: Query<(Entity, &Finger)>,
) {
    let mut rng = rand::rng();
    for (entity, finger) in finger_query.iter() {
        let ring_radius = 55.0;

        let countdown_ring_material = ring_material.add(CountdownMaterial {
            progress: 0.0,
            color: finger.color.to_linear(),
            secondary_color: finger.color.with_alpha(0.3).to_linear(),
            radius: ring_radius,
            texture: None,
            alpha_mode: AlphaMode2d::Blend,
        });

        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let rotation = Quat::from_rotation_z(-angle);

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                CountdownRing {
                    radius: ring_radius,
                    progress: 0.0,
                    complete: false,
                },
                Mesh2d(meshes.add(Circle::new(ring_radius + 10.0))),
                MeshMaterial2d(countdown_ring_material),
                Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(rotation),
            ));

            let x = angle.cos() * (ring_radius + 5.0);
            let y = angle.sin() * (ring_radius + 5.0);

            parent.spawn((
                GlowPoint { angle },
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(material.add(finger.color.with_luminance(0.8))),
                Transform::from_xyz(x, y, 0.0001)
            ));
        });
    }
}

pub fn animate_countdown_rings(
    time: Res<Time>,
    countdown: Res<CountdownTimer>,
    mut ring_query: Query<(&mut CountdownRing, &MeshMaterial2d<CountdownMaterial>)>,
    mut materials: ResMut<Assets<CountdownMaterial>>,
) {
    let progress = countdown.timer.elapsed_secs() / countdown.timer.duration().as_secs_f32();

    for (mut ring, material_handle) in ring_query.iter_mut() {
        ring.progress = progress;
        
        if let Some(material) = materials.get_mut(material_handle) {
            material.progress = progress;
        }
    }
}

pub fn animate_glow_points(
    time: Res<Time>,
    mut glow_query: Query<(&mut GlowPoint, &mut Transform)>
) {
    for (mut glow_point, mut transform) in glow_query.iter_mut() {
        glow_point.angle += time.delta_secs() * 2.0;

        let x = glow_point.angle.cos() * 60.0;
        let y = glow_point.angle.sin() * 60.0;
        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn cleanup_countdown_visuals(
    mut commands: Commands,
    ring_query: Query<Entity, With<CountdownRing>>,
    glow_query: Query<Entity, With<GlowPoint>>,
) {
    for entity in ring_query.iter() {
        commands.entity(entity).despawn();
    }

    for entity in glow_query.iter() {
        commands.entity(entity).despawn();
    }
}


