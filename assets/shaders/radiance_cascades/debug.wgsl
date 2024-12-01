#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return textureSampleLevel(screen_texture, screen_sampler, in.uv, 0.0);
    // return textureLoad(screen_texture, vec2<u32>(in.uv * vec2<f32>(1600.0, 900.0)), 0);
}
