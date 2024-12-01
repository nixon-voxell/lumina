@group(0) @binding(0) var<uniform> step_size: i32;
@group(0) @binding(1) var tex_jfa_source: texture_2d<u32>;
@group(0) @binding(2) var tex_jfa_destination: texture_storage_2d<rg16uint, write>;

const OFFSET_COUNT = 8;

@compute
@workgroup_size(8, 8, 1)
fn jfa(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>
) {
    let base_coord = vec2<i32>(global_id.xy);
    let base_coordf = vec2<f32>(base_coord);
    let dimension = vec2<i32>(textureDimensions(tex_jfa_source, 0));

    if any(base_coord >= dimension) {
        return;
    }

    var uv_offsets = array<vec2<i32>, OFFSET_COUNT>(
        vec2<i32>(-1, 1),
        vec2<i32>(0, 1),
        vec2<i32>(1, 1),
        vec2<i32>(-1, 0),
        vec2<i32>(1, 0),
        vec2<i32>(-1, -1),
        vec2<i32>(0, -1),
        vec2<i32>(1, -1),
    );

    var best_coord = textureLoad(tex_jfa_source, base_coord, 0).rg;
    let delta = vec2<f32>(best_coord) - base_coordf;
    var min_distance = dot(delta, delta);

    for (var i = 0; i < OFFSET_COUNT; i++) {
        let offset_coord = base_coord + uv_offsets[i] * step_size;
        if any(offset_coord >= dimension) || any(offset_coord < vec2<i32>(0)) {
            continue;
        }

        let offset_tex = textureLoad(tex_jfa_source, offset_coord, 0).rg;

        let delta = vec2<f32>(offset_tex) - base_coordf;
        let dist = dot(delta, delta);

        if dist < min_distance {
            min_distance = dist;
            best_coord = offset_tex;
        }
    }

    textureStore(
        tex_jfa_destination,
        base_coord,
        vec4<u32>(best_coord, 0, 0)
    );
}
