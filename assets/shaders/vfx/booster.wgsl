#import bevy_render::maths::HALF_PI
#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::perlin_noise_2d::perlinNoise2

@group(2) @binding(0) var<uniform> primary_color: vec4<f32>;
@group(2) @binding(1) var<uniform> secondary_color: vec4<f32>;
@group(2) @binding(2) var<uniform> rotation: f32;
@group(2) @binding(3) var<uniform> inv_scale: f32;
@group(2) @binding(4) var<uniform> ignition: f32;

fn rotate_uv(uv: vec2<f32>, rotation: f32, mid: vec2<f32>) -> vec2<f32>
{
    return vec2<f32>(
      cos(rotation) * (uv.x - mid.x) + sin(rotation) * (uv.y - mid.y) + mid.x,
      cos(rotation) * (uv.y - mid.y) - sin(rotation) * (uv.x - mid.x) + mid.y
    );
}

fn contract(x: f32, strength: f32, gradient: f32, gradient_power: f32) -> f32 {
    return mix(x, x * strength, pow(gradient, gradient_power));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Ignition gradient
    let ignition_grad = saturate(ignition - mesh.uv.y);
    // Rotate uv
    var uv = mesh.uv - 0.5;
    uv.y *= inv_scale;
    uv.y += inv_scale * 0.5 - 0.5;
    let rotated_uv = rotate_uv(uv, rotation, vec2<f32>(0.0, -0.5));

    uv = mix(uv, rotated_uv, smoothstep(0.2, 1.2, uv.y + 0.5));

    // Offset of the starting point the x uv contracts to form the sharp shape.
    let stretch_offset = 0.4 + (1.0 - ignition);

    // Noise
    var noise_uv = -uv;
    noise_uv.x = contract(noise_uv.x, 5.0, saturate(noise_uv.y + stretch_offset), 3.2);
    // Stretch y axis.
    noise_uv.y *= 0.1;
    noise_uv.y += globals.time * 0.3;

    let noise_multiplier = 30.0;
    let noise = perlinNoise2(noise_uv * noise_multiplier) * 0.5 + 0.5;

    // Shape.
    var shape_uv = uv;
    shape_uv.x *= 1.6;
    shape_uv.x = contract(shape_uv.x, 10.0, saturate(shape_uv.y + stretch_offset), 4.2);
    shape_uv.y += (noise - 0.5) * 0.13;

    let dist = 1.0 - length(shape_uv - vec2<f32>(0.0));

    let shape = smoothstep(0.5, 0.7, dist);
    let softer_shape = smoothstep(0.5, 0.9, dist);

    var effect = mix(noise, 1.0, shape);
    effect *= softer_shape;

    var col = mix(secondary_color, primary_color, pow(effect, 1.5)) * effect;
    col *= ignition_grad;
    col.a *= 1.0 + (1.0 - inv_scale) * 4.0;

    return col;
}
