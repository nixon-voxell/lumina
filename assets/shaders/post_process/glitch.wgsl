#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> config: GlitchConfig;

struct GlitchConfig {
    intensity: f32,
    time: f32,
}

// Hash function for randomness
fn hash(value: vec2<f32>) -> f32 {
    return fract(sin(dot(value, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Basic noise
fn noise(st: vec2<f32>) -> f32 {
    let i = floor(st);
    let f = fract(st);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    let orig_color = textureSample(screen_texture, texture_sampler, uv);

    if (config.intensity < 0.001) {
        return orig_color;
    }

    let time = config.time;
    let glitch_strength = config.intensity * 0.01;

    // Horizontal block-based distortion
    let block_size = 15.0;
    let block_uv = floor(uv * block_size) / block_size;
    let rand = hash(block_uv + vec2<f32>(floor(time), 0.0));
    let trigger = step(0.85 - config.intensity * 0.1, rand);

    if (trigger > 0.5) {
        let shift = (hash(vec2<f32>(block_uv.y, time)) * 2.0 - 1.0) * glitch_strength;
        uv.x = fract(uv.x + shift);
    }

    // Color channel drift with noise
    let drift = noise(uv * 50.0 + vec2<f32>(time, 0.0)) * glitch_strength;
    let r = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(drift, 0.0)).r;
    let g = textureSample(screen_texture, texture_sampler, uv).g;
    let b = textureSample(screen_texture, texture_sampler, uv - vec2<f32>(drift, 0.0)).b;

    let final_color = vec3<f32>(r, g, b);
    return vec4<f32>(final_color, orig_color.a);
}
