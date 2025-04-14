use bevy::{
    asset::RenderAssetUsages, prelude::*,
    render::mesh::{self, PrimitiveTopology, Indices}
};

use crate::{
    components::{
        Finger,
        CountdownRing,
        GlowPoint,
    },
    resources::CountdownTimer,
};

pub fn spawn_countdown_rings(
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
                Mesh2d(meshes.add(create_ring_mesh(ring_radius, 2.0, 0.0))),
                MeshMaterial2d(materials.add(finger.color.with_alpha(0.8))),
                Transform::from_xyz(0.0, 0.0, -0.1),
            ));

            parent.spawn((
                GlowPoint { angle: 0.0 },
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(finger.color.with_luminance(0.8))),
                Transform::from_xyz(ring_radius, 0.0, 0.0)
            ));
        });
    }
}

fn create_ring_mesh(radius: f32, thickness: f32, progress: f32) -> Mesh {
    let inner_radius = radius;
    let outer_radius = radius + thickness;

    let resolution = 36;
    
    let num_vertices = (resolution as usize + 1) * 2;
    let mut indices = Vec::with_capacity(resolution as usize * 6);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut uvs = Vec::with_capacity(num_vertices);
    let normals = vec![[0.0, 0.0, 1.0]; resolution];

    let start_angle = core::f32::consts::FRAC_PI_2;
    let step = core::f32::consts::TAU / resolution as f32;

    for i in 0..=resolution {
        let theta = start_angle + (i % resolution) as f32 * step;
        let (sin, cos) = ops::sin_cos(theta);
        let inner_pos = [cos * inner_radius, sin * inner_radius, 0.];
        let outer_pos = [cos * outer_radius, sin * outer_radius, 0.];
        positions.push(inner_pos);
        positions.push(outer_pos);

        let inner_uv = [0., i as f32 / resolution as f32];
        let outer_uv = [1., i as f32 / resolution as f32];
        uvs.push(inner_uv);
        uvs.push(outer_uv);
    }

    for i in 0..resolution as u32 {
        let inner_vertex: u32 = 2 * i;
        let outer_vertex: u32 = 2 * i + 1;
        let next_inner: u32 = inner_vertex + 2;
        let next_outer: u32 = outer_vertex + 2;

        indices.extend_from_slice(&[inner_vertex, outer_vertex, next_outer]);
        indices.extend_from_slice(&[next_outer, next_inner, inner_vertex]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList, 
        RenderAssetUsages::default()
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn animate_countdown_rings(
    time: Res<Time>,
    countdown_timer: Res<CountdownTimer>,
) {
   todo!("Animate Countdown Rings"); 
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


