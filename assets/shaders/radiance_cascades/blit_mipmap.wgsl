#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var tex_screen: texture_2d<f32>;
@group(0) @binding(1) var sampler_screen_filter: sampler;
@group(0) @binding(2) var sampler_screen_nearest: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var filtered_color = textureSample(tex_screen, sampler_screen_filter, in.uv);
    let nearest_color = textureSample(tex_screen, sampler_screen_nearest, in.uv);
    var alpha = filtered_color.a;
    filtered_color.a = alpha * 1.2;

    return filtered_color;
}

// @compute
// @workgroup_size(8, 8, 1)
// fn blur_mipmap(
//     @builtin(global_invocation_id) global_id: vec3<u32>,
//     @builtin(local_invocation_id) local_id: vec3<u32>,
// ) {
//     let base_coord = global_id.xy;
//     let dimensions = textureDimensions(tex_screen);

//     if any(base_coord >= dimensions) {
//         return;
//     }
// }
