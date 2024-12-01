#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var tex_radiance_mipmap1: texture_2d<f32>;
@group(0) @binding(1) var sampler_radiance_mipmap: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex_radiance_mipmap1, sampler_radiance_mipmap, in.uv);
}
