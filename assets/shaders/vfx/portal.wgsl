#import bevy_render::maths::HALF_PI
#import bevy_sprite::mesh2d_view_bindings::{globals, view}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::perlin_noise_2d::perlinNoise2

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var screen_texture: texture_2d<f32>;
@group(2) @binding(3) var screen_sampler: sampler;

fn coords_to_viewport_uv(position: vec2<f32>, viewport: vec4<f32>) -> vec2<f32> {
    var pos = position;
    pos.y = -pos.y;
    return pos / viewport.zw + 0.5;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (mesh.uv - 0.5) * 2.0;
    var viewport_uv = coords_to_viewport_uv(mesh.world_position.xy, view.viewport);

    let sdf_center = time * normalize(uv);

    let diff = sdf_center - uv;
    let len = length(diff);
    let fade_mask = pow(max(1.0 - dot(uv, uv), 0.0), 1.5);
    var distort_direction = normalize(diff);

    var d = length(diff);
    let color_mask = pow(max(1.0 - d, 0.0), 6.0);
    d *= 1. - smoothstep(0., 0.1, abs(d)); // Smooth the ripple
    let dir = normalize(diff);
    viewport_uv += dir * d * fade_mask * 0.5;

    let screen_color = textureSample(screen_texture, screen_sampler, viewport_uv);
    let final_color = screen_color + color * color_mask * fade_mask;

    return final_color;
}
