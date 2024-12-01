@group(0) @binding(0) var tex_mask: texture_2d<u32>;
@group(0) @binding(1) var tex_jfa: texture_storage_2d<rg16uint, write>;

@compute
@workgroup_size(8, 8, 1)
fn jfa_mask(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let base_coordinates = vec2<u32>(global_id.xy);

    let mask = textureLoad(tex_mask, base_coordinates, 0).r;

    if mask == 1u {
        textureStore(tex_jfa, base_coordinates, vec4<u32>(base_coordinates, 0, 0));
    } else {
        // Set to a far distance
        let far_coordinate = textureDimensions(tex_jfa) * 3u;
        textureStore(tex_jfa, base_coordinates, vec4<u32>(far_coordinate, 0, 0));
    }
}
