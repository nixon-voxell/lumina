// Inspirations from : https://www.shadertoy.com/view/lcKGRc

#import bevy_enoki::particle_vertex_out::{ VertexOutput }

fn gyroid(p: vec3<f32>) -> f32 { return dot(cos(p),sin(p.yzx)); }

fn fbm(_p: vec3<f32>) -> f32 {
    var p = _p;
    var result = 0.0;
    var a = 0.5;

    for (var i = 0; i < 3; i++) {
        p.z += (result) * 0.1;
        result += abs(gyroid(p / a) * a);
        a /= 1.7;
    }
    return result;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv - vec2<f32>(0.5, 0.5);

    // Animations
    let timeline = in.lifetime_frac;
    let growth = pow(timeline, 0.2);
    let fade = 1.0 - pow(timeline, 9.0);
    let scale = timeline;
    let burn = 1.0 - pow(timeline, 0.4);
    let speed = pow(timeline, 0.4);
    
    // Coordinates
    var ray = normalize(vec3<f32>(uv, 0.01 + scale));
    ray.z += speed + in.lifetime_total * 1000.0;
    
    // The spice
    let noise = fbm(ray);
    
    // Surface orientation
    let e = vec3<f32>(0.2, 0.2, 0.0);
    let normal = normalize(
        noise - vec3<f32>(fbm(ray + e.xzz), fbm(ray + e.zyz) , 1.0)
    );
    
    // Color with normals.
    var color = in.color.rgb * pow(cos(normal.y), 10.0);
    
    // Gradient blending and masking.
    let smoke = noise - 2.0 * burn;
    let shade = (normal.y * 0.5 + 0.5);
    color = mix(color, vec3(smoke * shade), smoothstep(0.0 , 0.1 , smoke));
    
    // shape
    let radius = 0.3 * noise * growth;
    let shape = smoothstep(.05,.0,length(uv)-radius);
    
    return vec4<f32>(color, shape * fade);
}
