#import bevy_render::maths::HALF_PI
#import bevy_sprite::mesh2d_view_bindings::{globals, view}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::simplex_noise_3d::simplexNoise3

@group(2) @binding(0) var<uniform> _color: vec4<f32>;

fn fbm(_x: vec3<f32>, it: u32) -> f32 {
    var x = _x;
    var v = 0.0;
    var a = 0.5;
    let shift = vec3<f32>(100.0);

    
    for (var i = 0u; i < it; i++) {
        v += a * simplexNoise3(x);
        x = x * 2.0 + shift;
        a *= 0.5;
    }
    return v;
}

fn rotateZ(v: vec3<f32>, angle: f32) -> vec3<f32> {
    let cos_angle = cos(angle);
    let sin_angle = sin(angle);
    return vec3<f32>(
        v.x * cos_angle - v.y * sin_angle,
        v.x * sin_angle + v.y * cos_angle,
        v.z
    );
}

fn facture(vector: vec3<f32>) -> f32 {
    let normalized_vector = normalize(vector);

    return max(max(normalized_vector.x, normalized_vector.y), normalized_vector.z);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (mesh.uv - 0.5) * 2.0;

    var color = vec3<f32>(uv, 0.5);
    color = normalize(color);
    color -= 0.2 * vec3<f32>(0.0, 0.0, globals.time);

    let angle = -log2(length(uv));
    color = rotateZ(color, angle);

    let frequency = 1.0;
    let distortion = 0.01;

    color.x = fbm(color * frequency + 0.0, 3u) + distortion;
    color.y = fbm(color * frequency + 1.0, 3u) + distortion;
    color.z = fbm(color * frequency + 2.0, 3u) + distortion;

    var fac = facture(color + 0.32) - length(uv) - 0.1;
    fac += 0.2;
    
    color = _color.rgb * fac;

    return vec4<f32>(color, fac * smoothstep(0.9, 0.8, length(uv)));
}
