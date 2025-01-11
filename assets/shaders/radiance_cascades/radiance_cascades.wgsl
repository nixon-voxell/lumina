#import bevy_render::maths::{PI_2, HALF_PI}
#import "shaders/radiance_cascades/radiance_probe.wgsl"::Probe;

const QUARTER_PI: f32 = HALF_PI * 0.5;
/// Raymarch length in pixels.
const RAYMARCH_LENGTH: f32 = 0.5;
const MAX_RAYMARCH: u32 = 128;
const EPSILON: f32 = 4.88e-04;

@group(0) @binding(0) var<uniform> num_cascades: u32;
@group(0) @binding(1) var<uniform> probe: Probe;
@group(0) @binding(2) var tex_main: texture_2d<f32>;
@group(0) @binding(3) var sampler_main: sampler;

@group(1) @binding(0) var tex_radiance_cascades_source: texture_2d<f32>;
@group(1) @binding(1) var tex_radiance_cascades_destination: texture_storage_2d<rgba16float, write>;

@compute
@workgroup_size(8, 8, 1)
fn radiance_cascades(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
) {
    let base_coord = global_id.xy;
    let dimensions = textureDimensions(tex_radiance_cascades_source);

    if any(base_coord >= dimensions) {
        return;
    }

    // Coordinate inside the probe grid
    let probe_texel = base_coord % probe.width;

    let ray_index = probe_texel.x + probe_texel.y * probe.width;
    let ray_count = probe.width * probe.width;

    var ray_angle = (f32(ray_index) + 0.5) / f32(ray_count) * PI_2;
    let ray_dir = normalize(vec2<f32>(cos(ray_angle), sin(ray_angle)));

    // Coordinate of cell in probe grid
    let probe_cell = base_coord / probe.width;
    // Start coordinate of the probe grid (in texture space)
    let probe_coord = probe_cell * probe.width;

    // Center coordinate of the probe grid
    let probe_origin = vec2<f32>(probe_coord + probe.width / 2);

    // let target_index = 5u;
    // var color = vec4<f32>(0.0);
    // if probe.cascade_index == target_index {
    //     color = raymarch(probe_origin, ray_dir);
    // }

    var color = raymarch(probe_origin, ray_dir);


#ifdef MERGE
    // TODO: Factor in transparency.
    if (color.a != 0.0) {
        let merge_color = merge(probe_cell, probe_coord, ray_index);
        color = mix(color, merge_color, color.a);
    }
#endif

    textureStore(
        tex_radiance_cascades_destination,
        base_coord,
        color
    );
}

fn raymarch(origin: vec2<f32>, ray_dir: vec2<f32>) -> vec4<f32> {
    var radiance = vec3<f32>(0.0);
    var visibility = 1.0;

    var volumetric_color = vec3<f32>(1.0);
    var position = origin;
    var covered_range = 0.0;

    let level_idx = f32(probe.cascade_index);
    let mip_step_size = f32(1u << probe.cascade_index);
    // let mip_step_size = 1.0;
    let dimensions = vec2<f32>(textureDimensions(tex_main));

    var march_count = 0u;
    let step_size = RAYMARCH_LENGTH * mip_step_size;
    // let ray_start = probe.start;
    let ray_start = probe.start * 0.5;
    let ray_end = probe.start + probe.range;

    for (var r = ray_start; r < ray_end; r += step_size) {
        let p = origin + ray_dir * r;
        if (any(p >= dimensions) || any(p < vec2<f32>(0.0))) {
            break;
        }

        // Prevent infinite loop.
        march_count++;
        if march_count > MAX_RAYMARCH {
            break;
        }

        let t = clamp((r - ray_start) / (ray_end - ray_start), 0.0, 1.0);
        let s = 0.85; let e = 0.0;
        let start_edge = clamp(((1.0 - t) - s) / (1.0 - s), 0.0, 1.0);
        let end_edge = clamp((t - e) / (1.0 - e), 0.0, 1.0);

        let coord = p / dimensions;
        var color = sample_main(coord, level_idx);

        var new_rad = mix(color.rgb, vec3<f32>(0.0), start_edge);
        new_rad = mix(new_rad, vec3<f32>(0.0), end_edge);

        var new_viz = mix(color.a, 1.0, start_edge);
        new_viz = mix(new_viz, 1.0, end_edge);

        let level_idx_1 = level_idx;
        radiance += new_rad * visibility * level_idx_1;
        visibility *= pow(new_viz, (level_idx_1 * 0.1));
    }

    return vec4<f32>(radiance, visibility);
}

fn sample_main(coord: vec2<f32>, level: f32) -> vec4<f32> {
    var color = textureSampleLevel(tex_main, sampler_main, coord, level - 1.0);
    let alpha = 1.0 - clamp(color.a, 0.0, 1.0);
    // Treat values from -1.0 ~ 1.0 as no light
    // This way, we can handle both negative and postive light
    let color_sign = sign(color.rgb);
    let color_abs = abs(color.rgb);

    let remaped_color = color_sign * max(color_abs - 1.0, vec3<f32>(0.0));

    return vec4<f32>(remaped_color, alpha);
}

fn merge(probe_cell: vec2<u32>, probe_coord: vec2<u32>, ray_index: u32) -> vec4<f32> {
    let dimensions = textureDimensions(tex_radiance_cascades_source);
    let prev_width = probe.width * 2;

    var TL = vec4<f32>(0.0);
    var TR = vec4<f32>(0.0);
    var BL = vec4<f32>(0.0);
    var BR = vec4<f32>(0.0);

    let probe_cell_i = vec2<i32>(probe_cell);
    let probe_correcetion_offset = probe_cell_i - probe_cell_i / 2 * 2;

    let prev_ray_index_start = ray_index * 4;
    for (var p: u32 = 0; p < 4; p++) {
        let prev_ray_index = prev_ray_index_start + p;

        let offset_coord = vec2<u32>(
            prev_ray_index % prev_width,
            prev_ray_index / prev_width,
        );

        TL += fetch_cascade(
            probe_cell_i,
            probe_correcetion_offset + vec2<i32>(-1, -1),
            offset_coord,
            dimensions,
            prev_width
        );
        TR += fetch_cascade(
            probe_cell_i,
            probe_correcetion_offset + vec2<i32>(0, -1),
            offset_coord,
            dimensions,
            prev_width
        );
        BL += fetch_cascade(
            probe_cell_i,
            probe_correcetion_offset + vec2<i32>(-1, 0),
            offset_coord,
            dimensions,
            prev_width
        );
        BR += fetch_cascade(
            probe_cell_i,
            probe_correcetion_offset + vec2<i32>(0, 0),
            offset_coord,
            dimensions,
            prev_width
        );
    }

    let weight = 0.75 - (
        vec2<f32>(probe_correcetion_offset) * 0.5
    );

    return mix(mix(TL, TR, weight.x), mix(BL, BR, weight.x), weight.y) * 0.25;
}

fn fetch_cascade(
    // Current probe's start coordinate
    probe_cell: vec2<i32>,
    probe_offset: vec2<i32>,
    offset_coord: vec2<u32>,
    dimensions: vec2<u32>,
    prev_width: u32,
) -> vec4<f32> {
    var prev_probe_cell = probe_cell / 2 + probe_offset;
    prev_probe_cell = clamp(prev_probe_cell, vec2<i32>(0), vec2<i32>(dimensions / prev_width - 1));

    let prev_probe_coord = vec2<u32>(prev_probe_cell) * prev_width + offset_coord;
    return textureLoad(tex_radiance_cascades_source, prev_probe_coord, 0);
}
