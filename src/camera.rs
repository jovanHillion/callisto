use cgmath::{Array, EuclideanSpace};

// #[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: cgmath::Vector3<f32>, // camera position
    pub look_at: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {

    pub fn new(config: &wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>>) -> Self {
        Self {
            eye: (0.0, 0.0, 20.0).into(),
            look_at: (0.0, 0.0, -1.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 60.0,
            znear: 0.1,
            zfar: 1000.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // From, to, pos
        let view = cgmath::Matrix4::look_at_rh(cgmath::Point3::from_vec(self.eye), cgmath::Point3::from_vec(self.eye + self.look_at), self.up);

        // Give the effect of depth
        let projection =
            cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * projection * view;
    }
}
