struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
}

// This struct store the output of the vertex shader (clip_position)
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates
    @location(1) color: vec3<f32>,
}


@vertex fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

////////////////////////////////////////////

// Vertex Shader
@vertex // @vertex to mark this function as a valid entry point for a vertex shader
fn vs_test_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput { // We expect a u32 called in_vertex_index, which gets its value from @builtin(vertex_index)
    var out: VertexOutput; // var must specify their types but are mutable
    let x = f32(1 - i32(in_vertex_index)) * 0.5; // let can inferred their types
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    out.clip_position = vec4<f32>(
        x, y, 0.0, 1.0,
    );

    return out;
}

/*
    Fragment Shader

    @builtin(position), in the fragment shader, this value is in framebuffer space (opens new window).
    This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600,
    respectively, with the y = 0 being the top of the screen
*/
@fragment fn fs_test_main(in: VertexOutput) -> @location(0) vec4<f32> { // The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target. We'll get into what this is later.
    return vec4<f32>(in.clip_position.r, in.clip_position.g, in.clip_position.b, in.clip_position.a); // Defines the color of the triangle
}
