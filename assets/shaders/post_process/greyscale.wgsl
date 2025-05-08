#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> config: GreyscaleConfig;

struct GreyscaleConfig {
    intensity: f32,
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, texture_sampler, in.uv);
    let grey = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let final_color = mix(color.rgb, vec3<f32>(grey, grey, grey), config.intensity);
    return vec4<f32>(final_color, color.a);
}