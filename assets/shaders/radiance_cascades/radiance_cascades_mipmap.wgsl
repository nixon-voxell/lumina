#import "shaders/radiance_cascades/radiance_probe.wgsl"::Probe;

@group(0) @binding(0) var<uniform> probe: Probe;
@group(0) @binding(1) var tex_radiance_cascades: texture_2d<f32>;
@group(0) @binding(2) var tex_radiance_mipmap: texture_storage_2d<rgba16float, write>;

@compute
@workgroup_size(8, 8, 1)
fn radiance_cascades_mipmap(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let base_coord = global_id.xy;
    let dimensions = textureDimensions(tex_radiance_mipmap);

    if any(base_coord >= dimensions) {
        return;
    }

    let probe_cell = base_coord * probe.width;
    let ray_count = probe.width * 2;

    var accumulation = vec4<f32>(0.0);
    for (var y: u32 = 0; y < probe.width; y++) {
        for (var x: u32 = 0; x < probe.width; x++) {
            accumulation += textureLoad(tex_radiance_cascades, probe_cell + vec2<u32>(x, y), 0);
        }
    }
    accumulation /= f32(ray_count);

    textureStore(tex_radiance_mipmap, base_coord, accumulation);
}
