#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const PI = radians(180.0);
const TAU = radians(360.0);

struct CountdownMaterial {
    progress: f32,
    color: vec4<f32>,
    secondary_color: vec4<f32>,
    radius: f32,
};

@group(2) @binding(0)
var<uniform> progress: f32;
@group(2) @binding(1)
var<uniform> color: vec4<f32>;
@group(2) @binding(2)
var<uniform> secondary_color: vec4<f32>;
@group(2) @binding(3)
var<uniform> radius: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);

    let angle = atan2(uv.y - 0.5, uv.x - 0.5) + PI;
    let normalized_angle = angle / TAU;

    let ring_radius = 0.5;
    let ring_thickness = 0.08;
    let inner_radius = ring_radius - ring_thickness;
    let outer_radius = ring_radius;
    let ring_mask = smoothstep(inner_radius, inner_radius + 0.01, dist) * (1.0 - smoothstep(outer_radius - 0.01, outer_radius, dist));

    let progress_mask = ex_mod(-normalized_angle - progress, 1.0);
    let progress_point = smoothstep(0.6 - progress, 1.0, progress_mask);

    let time = progress * 10.0;
    let pulse = (sin(time) * 0.1 + 0.9);

    let gradient = mix(secondary_color, color, progress_point);

    let progress_colors = mix(secondary_color * 0.7, gradient, progress_point);

    let glow_intensity = 0.3 + progress * 0.7;
    let glow_falloff = 0.1 + (1.0 - progress) * 0.2;
    let glow = color * pow(1.0 - abs(dist - radius) / glow_falloff, 2.0) * glow_intensity;

    let final_color = vec4(progress_colors.rgb, progress_point * ring_mask);
    return final_color;
}

fn ex_mod(a: f32, b: f32) -> f32 {
    return a - floor(a / b) * b;
}

fn parabola(x: f32, k: f32) -> f32 {
    return pow(4.0 * x * (1.0 - x), k);
}
