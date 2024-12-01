@group(0) @binding(0) var tex_jfa: texture_2d<u32>;
@group(0) @binding(1) var tex_dist_field: texture_storage_2d<r16float, write>;

@compute
@workgroup_size(8, 8, 1)
fn distance_field(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>
) {
    let base_coordinates = vec2<u32>(global_id.xy);
    let base_coordinatesf = vec2<f32>(base_coordinates);

    let jfa = vec2<f32>(textureLoad(tex_jfa, base_coordinates, 0).rg);

    textureStore(
        tex_dist_field,
        base_coordinates,
        vec4<f32>(distance(base_coordinatesf, jfa))
    );
}
