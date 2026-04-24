use cgmath::{Vector3};
use wgpu::VertexBufferLayout;

// Pod indicates that our Vertex is "Plain Old Data", and thus can be interpreted as a &[u8]
// Zeroable indicates that we can use std::mem::zeroed()
#[repr(C)] // Handling the memory the same way as C does, OpenGL may use the C ABI (align the memory like in c so when the programm allocates the memory it set the bits at the irght position)
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4
                },
            ]
        }
    }
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(Vector3 {
                x: 0.0 as f32,
                y: 0.0 as f32,
                z: 0.0 as f32,
            }) * (cgmath::Matrix4::from(self.rotation) * cgmath::Matrix4::from_translation(self.position))).into(),
        }
    }
}

impl Vertex {

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

struct Dimension(f32, f32, f32);

const DIMENSION: Dimension = Dimension(0.5, 0.5, 0.5);

// Vertex data
pub const VERTICES: &[Vertex] = &[

    // Front face
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [1.0, 1.0, 0.0] },
    Vertex { position: [DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [1.0, 1.0, 0.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [1.0, 1.0, 0.0] },
    Vertex { position: [-DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [1.0, 1.0, 0.0] },

    // Back face
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 0.0] },
    Vertex { position: [DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 0.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 0.0] },

    // Right face
    Vertex { position: [DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [1.0, 0.0, 1.0] },
    Vertex { position: [DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 1.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [1.0, 0.0, 1.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [1.0, 0.0, 1.0] },

    // Left face
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [0.0, 1.0, 0.0] },

    // Top face
    Vertex { position: [-DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [0.0, 1.0, 1.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, DIMENSION.2], color: [0.0, 1.0, 1.0] },
    Vertex { position: [DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-DIMENSION.0, DIMENSION.1, -DIMENSION.2], color: [0.0, 1.0, 1.0] },

    // Bottom face
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [0.0, 0.0, 1.0] },
    Vertex { position: [DIMENSION.0, -DIMENSION.1, DIMENSION.2], color: [0.0, 0.0, 1.0] },
    Vertex { position: [DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-DIMENSION.0, -DIMENSION.1, -DIMENSION.2], color: [0.0, 0.0, 1.0] }
];

// Indices data
pub const INDICES: &[u16] = &[

    // Front face
	0, 1, 2,
    0, 2, 3,

    // Back face
    4, 6, 5,
    4, 7, 6,

    // Right face
    8, 9, 10,
    8, 10, 11,

    // Left face
    12, 13, 14,
    12, 14, 15,

    // Top face
    16, 17, 18,
    16, 18, 19,

    // Bottom face
    20, 22, 21,
    20, 23, 22
];