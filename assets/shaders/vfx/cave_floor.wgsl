#import bevy_render::maths::HALF_PI
#import bevy_sprite::mesh2d_view_bindings::{globals, view}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::perlin_noise_2d::perlinNoise2

@group(2) @binding(0) var<uniform> color0: vec4<f32>;
@group(2) @binding(1) var<uniform> color1: vec4<f32>;

fn hash(p: vec2<f32>, seed: vec2<f32>) -> f32 {
    return fract(sin(dot(p, seed)) * 43758.5453);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    // Scale UV to tile more cells
    let size = 50.0;
    let scaled_uv = uv * size;

    let cell = floor(scaled_uv);
    let f = fract(scaled_uv);

    var min_dist = 100.0;
    var nearest_point = vec2<f32>(0.0, 0.0);

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let offset = cell + neighbor;

            let point = vec2<f32>(
                hash(offset, vec2<f32>(127.1, 311.7)),
                hash(offset, vec2<f32>(269.5, 183.3))
            );

            let diff = neighbor + point - f;
            let dist = dot(diff, diff);

            if (dist < min_dist) {
                min_dist = dist;
                nearest_point = offset + point;
            }
        }
    }

    // Normalize cell position for coloring
    let cell_pos = nearest_point / size;
    let noise_size = 7.0;

    let noise = pow(perlinNoise2(cell_pos * noise_size + globals.time * 0.1) * 0.5 + 0.5, 0.8);

    return vec4<f32>(vec3<f32>(mix(color1.rgb, color0.rgb, noise)), smoothstep(0.6, 1.0, noise) * 0.75);
}
