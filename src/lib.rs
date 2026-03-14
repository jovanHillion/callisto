// We have to import the DeviceExt (opens new window)extension trait. For more information on extension traits: http://xion.io/post/code/rust-extension-traits.html
use wgpu::util::DeviceExt;

// Pod indicates that our Vertex is "Plain Old Data", and thus can be interpreted as a &[u8]
// Zeroable indicates that we can use std::mem::zeroed()
#[repr(C)] // what this one do ?
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Vertex {

    // Another way of defining our VertexBufferLayout
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    // fn desc() -> wgpu::VertexBufferLayout<'static> {
    //     wgpu::VertexBufferLayout {
    //         array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // Actual total size of ONE Vertex, it means that if we have multiple Vertex and we want to read the next Vertex to draw it we will probably skip 'array_stride' number of bytes
    //         step_mode: wgpu::VertexStepMode::Vertex, // step_mode tells the pipeline whether each element of the array in this buffer represents per-vertex data or per-instance data
    //         // Vertex attributes describe the individual parts of the vertex. Generally, this is a 1:1 mapping with a struct's fields, which is true in our case
    //         attributes: &[
    //             wgpu::VertexAttribute {
    //                 /*
    //                     This defines the offset in bytes until the attribute starts. For the first attribute, the offset is usually zero.
    //                     For any later attributes, the offset is the sum over size_of of the previous attributes' data.

    //                     0 is the offest because we want to use the position of our Vertex
    //                 */
    //                 offset: 0,
    //                 /*
    //                     This defines the offset in bytes until the attribute starts. For the first attribute, the offset is usually zero.
    //                     For any later attributes, the offset is the sum over size_of of the previous attributes' data.

    //                     Use to refer to the shader wgsl functions -> @location(0) to handle the vertices
    //                 */
    //                 shader_location: 0,
    //                 /*
    //                     Tells the shader the shape of the attribute. Float32x3 corresponds to vec3<f32> in shader code.
    //                     The max value we can store in an attribute is Float32x4 (Uint32x4, and Sint32x4 work as well).

    //                     It is the actual shape of data that we are working on (each Vertex position is corresponding to a Float32x3 matrix)
    //                 */
    //                 format: wgpu::VertexFormat::Float32x3
    //             },
    //             wgpu::VertexAttribute {
    //                 offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // Here we want to use the color not the position, this is why we are using an offset of the size of the position
    //                 shader_location: 1, // use to refer to the shader wgsl functions -> @location(1) to handle the vertices
    //                 format: wgpu::VertexFormat::Float32x3,
    //             }
    //         ]
    //     }
    // }
}

// Vertex data
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];