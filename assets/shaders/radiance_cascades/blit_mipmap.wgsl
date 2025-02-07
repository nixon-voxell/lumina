#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var tex_screen: texture_2d<f32>;
@group(0) @binding(1) var sampler_screen_filter: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dimensions = vec2<f32>(textureDimensions(tex_screen));
    let pix = (vec2<f32>(1.0) / dimensions) * 1.0;

    let center = textureSample(tex_screen, sampler_screen_filter, in.uv);
    let TL = textureSample(tex_screen, sampler_screen_filter, in.uv - pix);
    let TR = textureSample(tex_screen, sampler_screen_filter, in.uv + vec2<f32>(pix.x, -pix.y));
    let BL = textureSample(tex_screen, sampler_screen_filter, in.uv + vec2<f32>(-pix.x, pix.y));
    let BR = textureSample(tex_screen, sampler_screen_filter, in.uv + pix);

    // let color = (center + TL + TR + BL + BR) * 0.2;
    let color = center * 0.3 + (TL + TR + BL + BR) * 0.175;

    return color;
}

@compute
@workgroup_size(8, 8, 1)
fn blur_mipmap(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
) {
    let base_coord = global_id.xy;
    let dimensions = textureDimensions(tex_screen);

    if any(base_coord >= dimensions) {
        return;
    }
}
