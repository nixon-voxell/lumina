#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var tex_screen: texture_2d<f32>;
@group(0) @binding(1) var sampler_screen_filter: sampler;
@group(0) @binding(2) var sampler_screen_nearest: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var filtered_color = textureSample(tex_screen, sampler_screen_filter, in.uv);
    let nearest_color = textureSample(tex_screen, sampler_screen_nearest, in.uv);
    filtered_color.a = nearest_color.a;

    return filtered_color;
}
