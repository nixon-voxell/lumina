#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var tex_screen: texture_2d<f32>;
@group(0) @binding(1) var sampler_screen: sampler;
@group(0) @binding(2) var tex_radiance: texture_2d<f32>;
@group(0) @binding(3) var sampler_radiance: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var main = textureSample(tex_screen, sampler_screen, in.uv);
    main = max(vec4<f32>(main.rgb - 1.0, main.a), vec4<f32>(0.0));
    let radiance = textureSample(tex_radiance, sampler_radiance, in.uv);

    return main + radiance;
}
