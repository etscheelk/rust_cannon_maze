struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct MyUniform
{
    color: vec4f
}

@group(3) @binding(0)
var<uniform> my_uniform: MyUniform;
// var<uniform> color: vec4f;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f
{
    // return vec4f(1.0, 0.5, 0.5, 1.0);
    // if (in.position.x < 20.0 && in.position.y < 20.0)
    // {
    //     // return vec4f(0.0, 0.0, 1.0, 0.5);
    //     return in.color;
    // }
    
    // var can be modified
    // var centered = in.uv * 2.0 - 1.0;

    // let a = length(centered);

    // if a < 0.5
    // {
    //     return vec4f(1.0);
    // }
    // else
    // {
    //     return vec4f(0.0, 0.0, 0.0, 1.0);
    // }
    

    // return vec4f(in.position.xyz / 400.0, 0.5);

    // return my_uniform.color;
    // return my_uniform.color;
    return vec4f(in.uv, 1.0, 0.5);
}