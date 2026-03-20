// Pod indicates that our Vertex is "Plain Old Data", and thus can be interpreted as a &[u8]
// Zeroable indicates that we can use std::mem::zeroed()
#[repr(C)] // Handling the memory the same way as C does, OpenGL may use the C ABI (align the memory like in c so when the programm allocates the memory it set the bits at the irght position)
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
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