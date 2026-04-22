struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> origin: vec3<f32>;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

// This struct store the output of the vertex shader (clip_position)
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates
    @location(1) color: vec3<f32>,
}


@vertex fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>(model.position + origin, 1.0);
    out.color = model.color;

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
