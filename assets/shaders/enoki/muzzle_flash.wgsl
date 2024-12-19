// Inspirations from : https://www.shadertoy.com/view/DsVSzG

#import bevy_enoki::particle_vertex_out::{ VertexOutput }
#import bevy_shader_utils::perlin_noise_2d::perlinNoise2

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let fire_range = 0.6;
    var out = in.color;

    let diff = in.uv - center;
    let distance_from_center = length(diff) * 2.0;

    var x_angle = atan2(diff.y, diff.x) / 3.142;
    var angle = atan2(diff.y, diff.x) / 3.142;
    let variation = in.lifetime_total * 10000.0;
    let noise = perlinNoise2(vec2<f32>(x_angle * 20.0, variation + in.lifetime_frac));
    angle = 1.0 - abs(angle);
    angle = saturate(angle - 0.8) / 0.2;

    var inner_ring_pos = 0.1;
    let outer_ring_pos = 0.8;

    inner_ring_pos -= noise * 6.0 * (0.9 - outer_ring_pos);
    let inside_gradient = (distance_from_center - inner_ring_pos) / outer_ring_pos;

    out.a = pow(1.0 - saturate(inside_gradient - inner_ring_pos), 10.0) * angle;

    let outer_radius = saturate(pow(1.0 - saturate(dot(diff, diff)), 30.0));
    let inner_radius = saturate(pow(outer_radius, 4.0)) * angle;

    out.a *= outer_radius;
    out.a += inner_radius;

    return out;
}
