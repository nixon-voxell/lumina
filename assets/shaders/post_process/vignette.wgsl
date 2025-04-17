#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct VignetteConfig {
    intensity: f32,
    distance: f32,
    tint: vec3<f32>,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> config: VignetteConfig;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let diff = in.uv - 0.5;
    let strength = pow(
        clamp(dot(diff, diff) - config.distance, 0.0, 1.0),
        config.intensity
    );
    let tint = mix(vec3<f32>(1.0), config.tint, strength);

    return textureSample(screen_texture, texture_sampler, in.uv) *
        vec4<f32>(tint, 1.0) + vec4<f32>(config.tint * 0.1, 1.0) * strength;
}
