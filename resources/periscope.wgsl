struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct PeriscopeUniform
{
    position: vec2<f32>,
    width: f32
}

@group(3) @binding(0)
var<uniform> ps: PeriscopeUniform;

/*
    Acts as a spotlight around given position
    In reality, it darkest everything around it

    Blend mode should be on multiply
*/
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    let d = distance(ps.position, in.position.xy);

    var col = vec3f(1.0);

    if d > 100.0 // HARDCODED, FIXME
    {
        col = vec3f(0.005);
    }

    // let out = vec4<f32>(col, 1.0);
    return vec4f(col, 0.0);
}