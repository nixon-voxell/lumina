#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct ChromaticAberrationConfig {
    intensity: f32,
    distance: f32,
    contrast: f32,
    samples: u32,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> config: ChromaticAberrationConfig;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dimensions = vec2<f32>(textureDimensions(screen_texture));

    let diff = in.uv - 0.5;
    let strength = pow(
        clamp(dot(diff, diff) - config.distance, 0.0, 1.0),
        config.intensity
    );

    // Implementation from <https://www.shadertoy.com/view/DtGSRt>
    var color_sum = vec4<f32>(0.0);
    var weight_sum = vec4<f32>(0.0);

    for (var s = 0u; s < config.samples; s += 1u) {
        let i = f32(s) / f32(config.samples - 1);
        // var coord = in.uv;

        // Twist method.
        // let ratio = dimensions.yx / dimensions.y;
        // let coord = in.uv + vec2<f32>(in.uv.y - 0.5, 0.5 - in.uv.x) *
        //     ratio * ratio * (i - 0.5) * 0.1 * strength;

        // Linear method.
        let coord = mix(in. uv, vec2<f32>(0.5), (i - 0.5) * 0.1 * strength);

        var weight = vec4<f32>(i, 1.0 - abs(i + i - 1.0), 1.0 - i, 0.5);
        weight = mix(vec4<f32>(0.5), weight, config.contrast);
        
        let color = textureSample(screen_texture, texture_sampler, coord);
		color_sum += color * color * weight;
        //This makes each sample have a different color from red to green to blue
        //The total should be multiplied by the 2/number of samples, (e.g. 0.1)
        weight_sum += weight;
    }

    return sqrt(color_sum / weight_sum);
}
