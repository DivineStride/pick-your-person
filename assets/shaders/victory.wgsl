#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> color: vec4<f32>;
@group(2) @binding(1)
var<uniform> time: f32;
@group(2) @binding(2)
var<uniform> center: vec2<f32>;
@group(2) @binding(3)
var<uniform> ripple_speed: f32;
@group(2) @binding(4)
var<uniform> ripple_intensity: f32;
@group(2) @binding(5)
var<uniform> aspect_ratio: f32;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(in.uv.x * aspect_ratio, in.uv.y);

    // Calculate distance from winner's position
    let dist = distance(uv, center);

    let ripple_time = time * ripple_speed;
    let ripple1 = sin(dist * 20.0 - ripple_time) * 0.5 + 0.5;
    let ripple2 = sin(dist * 15.0 - ripple_time * 0.7) * 0.5 + 0.5;
    let ripple3 = sin(dist * 10.0 - ripple_time * 0.5) * 0.5 + 0.5;

    let ripple = (ripple1 * 0.4 + ripple2 * 0.3 + ripple3 * 0.3) * ripple_intensity;

    let wave_radius = sin(dist * 20.0 - ripple_time) * 0.5 + 0.5;
    let wave_thickness = 0.05;
    let wave = smoothstep(wave_radius - wave_thickness, wave_radius, dist) * (1.0 - smoothstep(wave_radius, wave_radius + wave_thickness, dist));

    let center_intensity = smoothstep(1.0, 0.0, dist * 2.0);

    var final_color = color.rgb;

    final_color *= vec3<f32>(smoothstep(0.3, 0.5, ripple) + ripple);

    final_color = mix(final_color, vec3<f32>(1.0, 1.0, 1.0), center_intensity * 0.5);

    return vec4<f32>(final_color, smoothstep(0.1, 0.5, sin(dist - time * 0.2) + 0.5));
}
