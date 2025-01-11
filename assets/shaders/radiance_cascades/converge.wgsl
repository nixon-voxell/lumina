@group(0) @binding(0) var<uniform> probe_width: u32;
@group(0) @binding(1) var tex_radiance_cascades: texture_2d<f32>;
@group(0) @binding(2) var tex_converge: texture_storage_2d<rgba16float, write>;

@compute
@workgroup_size(8, 8, 1)
fn converge(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
) {
    let base_coord = global_id.xy;
    let dimensions = textureDimensions(tex_radiance_cascades);

    if any(base_coord >= dimensions) {
        return;
    }

    let probe_cell = base_coord * probe_width;
    var color = vec4<f32>(0.0);
    for (var y = 0u; y < probe_width; y++) {
        for (var x = 0u; x < probe_width; x++) {
            color += textureLoad(tex_radiance_cascades, probe_cell + vec2<u32>(x, y), 0);
        }
    }

    textureStore(tex_converge, base_coord, color * 0.25);
}
