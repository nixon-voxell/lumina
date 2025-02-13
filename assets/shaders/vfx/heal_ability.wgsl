#import bevy_render::maths::{HALF_PI, PI_2}
#import bevy_sprite::mesh2d_view_bindings::{globals, view}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::perlin_noise_2d::perlinNoise2

@group(2) @binding(0) var<uniform> color0: vec4<f32>;
@group(2) @binding(1) var<uniform> color1: vec4<f32>;
@group(2) @binding(2) var<uniform> time: f32;
@group(2) @binding(3) var screen_texture: texture_2d<f32>;
@group(2) @binding(4) var screen_sampler: sampler;

fn coords_to_viewport_uv(position: vec2<f32>, viewport: vec4<f32>) -> vec2<f32> {
    var pos = position;
    pos.y = -pos.y;
    // return pos / viewport.zw + 0.5;
    // Our viewport ratio is fixed.
    return pos / vec2<f32>(1280.0, 720.0) + 0.5;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (mesh.uv - 0.5) * 2.0;
    let viewport_uv = coords_to_viewport_uv(
        mesh.world_position.xy - view.world_position.xy,
        view.viewport
    );

    // Fading at the edge
    let uv_sqr = dot(uv, uv);
    let fade_mask = max(1.0 - uv_sqr, 0.0);

    let uv_dir = normalize(uv);
    let target_dist = mix(-1.0, 1.0, time);
    let sdf_center = target_dist * uv_dir;

    let diff = sdf_center - uv;
    let len = length(diff);
    var distort_direction = normalize(diff);

    var d = length(diff) - 0.3;
    // Smooth the ripple.
    d *= 1. - smoothstep(0.0, 0.1, abs(d));
    let dir = normalize(diff);
    let distorted_viewport_uv = viewport_uv + dir * d * fade_mask;

    let distorted_screen_color = textureSample(screen_texture, screen_sampler, distorted_viewport_uv);
    let screen_color = textureSample(screen_texture, screen_sampler, viewport_uv);

    let color_mask = pow(clamp(1.3 - length(uv - sdf_center), 0.0, 1.0), 3.0);

    let angle = atan2(uv_dir.x, uv_dir.y);
    let color = mix(color0, color1, dot(uv_dir, vec2<f32>(1.0, 0.0)) * 0.5 + 0.5);
    let ring_color = color.rgb * color_mask;

    var final_color = distorted_screen_color.rgb * ring_color * 5.0 + ring_color;

    return vec4<f32>(final_color, min(screen_color.a, color_mask * fade_mask));
}
