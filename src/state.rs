use std::sync::Arc;


use winit::{
    event::*,
    event_loop::{
        ActiveEventLoop,
    },
    keyboard::{
        KeyCode,
    },
    window::Window
};

use rand::{
    RngExt,
    rngs::ThreadRng
};

use wgpu::{util::DeviceExt};

use crate::camera::{self};
use crate::camera_controller::{self};
use callisto::{
    INDICES, VERTICES, Vertex
};


// This will store the state of our game
pub struct State {
    pub window: Arc<Window>, // Arc is equivalent to a Shared ptr atomic
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    render_pipeline: wgpu::RenderPipeline,
    second_render_pipeline: wgpu::RenderPipeline,
    pipeline_state: bool,

    // Vertex Buffer
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    // test
    bckg_color: wgpu::Color,
    rng: ThreadRng,

    camera: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: camera_controller::CameraController
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),                     // allows us to specify what extra features we want, no specific extra features in my case
            experimental_features: wgpu::ExperimentalFeatures::disabled(),  // specifies whether we intend to use features that are not stable yet
            required_limits: wgpu::Limits::defaults(),                      // describes the limit of certain types of resources that we can create
            memory_hints: Default::default(),                               // provides the adapter with a preferred memory allocation strategy
            trace: wgpu::Trace::Off,
        }).await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
        .find(|f| f.is_srgb()).copied().unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,  // escribes how SurfaceTextures will be used. RENDER_ATTACHMENT specifies that the textures will be used to write to the screen
            format: surface_format,                         // defines how SurfaceTextures will be stored on the GPU. We can get a supported format from the SurfaceCapabilities
            width: size.width,                              // width and height are the width and the height in pixels of a SurfaceTexture. This should usually be the width and the height of the window
            height: size.height,                            // make sure that the width and height of the SurfaceTexture are not 0, as that can cause your app to crash.
            present_mode: surface_caps.present_modes[0],    // present_mode uses wgpu::PresentMode enum, which determines how to sync the surface with the display. For the sake of simplicity, we select the first available option. If you do not want runtime selection, PresentMode::Fifo will cap the display rate at the display's framerate. This is essentially VSync (https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html)
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let default_color = wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let camera = camera::Camera {
            eye: (0.0, 1.0, 2.0).into(), 
            look_at: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
        };
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_controller = camera_controller::CameraController::new(0.008);

        // Uniform buffer -> Create a bind group with it
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camrea Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX, // as we only really need camera information in the vertex shader, to manipulate our vertices
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    }
                ]
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ]
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                ],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),               // Specifies the entry point
                buffers: &[
                    Vertex::desc(),
                ],                               // Here we are giving the buffer of vertices we want to draw, for now we are creating it in the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {            // The fragment is technically optional, so you have to wrap it in Some(). We need it if we want to store color data to the surface
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {    // Tells wgpu what color outputs it should set up. Currently, we only need one for the surface.
                    format: config.format,                  // We use the surface's format so that copying to it is easy,
                    blend: Some(wgpu::BlendState::REPLACE), // Specifies that the blending should just replace old pixel data with new data
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // The primitive field describes how to interpret our vertices when converting them into triangles
            // The front_face and cull_mode fields tell wgpu how to determine whether a given triangle is facing forward or not
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Using PrimitiveTopology::TriangleList means that every three vertices will correspond to one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,   // FrontFace::Ccw means that a triangle is facing forward if the vertices are arranged in a counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),  // Triangles that are not considered facing forward are culled (not included in the render) as specified by CullMode::Bac
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,                    // We're not using a depth/stencil buffer currently, so we leave depth_stencil as None
            multisample: wgpu::MultisampleState {
                count: 1,                           // Count determines how many samples the pipeline will use
                mask: !0,                           // Mask specifies which samples should be active. In this case, we are using all of them
                alpha_to_coverage_enabled: false,   // alpha_to_coverage_enabled has to do with anti-aliasing. We're not covering anti-aliasing here
            },
            multiview_mask: None,                   // Multiview indicates how many array layers the render attachments can have. We won't be rendering to array textures, so we can set this to None
            cache: None,                            // cache allows wgpu to cache shader compilation data. Only really useful for Android build targets

        });

        let second_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_test_main"),               // Specifies the entry point
                buffers: &[],                               // Here we are giving the buffer of vertices we want to draw, for now we are creating it in the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {            // The fragment is technically optional, so you have to wrap it in Some(). We need it if we want to store color data to the surface
                module: &shader,
                entry_point: Some("fs_test_main"),
                targets: &[Some(wgpu::ColorTargetState {    // Tells wgpu what color outputs it should set up. Currently, we only need one for the surface.
                    format: config.format,                  // We use the surface's format so that copying to it is easy,
                    blend: Some(wgpu::BlendState::REPLACE), // Specifies that the blending should just replace old pixel data with new data
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // The primitive field describes how to interpret our vertices when converting them into triangles
            // The front_face and cull_mode fields tell wgpu how to determine whether a given triangle is facing forward or not
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Using PrimitiveTopology::TriangleList means that every three vertices will correspond to one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,   // FrontFace::Ccw means that a triangle is facing forward if the vertices are arranged in a counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),  // Triangles that are not considered facing forward are culled (not included in the render) as specified by CullMode::Bac
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,                    // We're not using a depth/stencil buffer currently, so we leave depth_stencil as None
            multisample: wgpu::MultisampleState {
                count: 1,                           // Count determines how many samples the pipeline will use
                mask: !0,                           // Mask specifies which samples should be active. In this case, we are using all of them
                alpha_to_coverage_enabled: false,   // alpha_to_coverage_enabled has to do with anti-aliasing. We're not covering anti-aliasing here
            },
            multiview_mask: None,                   // Multiview indicates how many array layers the render attachments can have. We won't be rendering to array textures, so we can set this to None
            cache: None,                            // cache allows wgpu to cache shader compilation data. Only really useful for Android build targets

        });

        // Vertex Buffer
        // The create_buffer_init() method expects a &[u8], so we are using Bytemuck to make it for us
        let vertex_buffer = device.create_buffer_init( &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index Buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX
        });

        let num_indices= INDICES.len() as u32;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            render_pipeline,
            second_render_pipeline,
            pipeline_state: false,
            vertex_buffer,
            index_buffer,
            num_indices,
            bckg_color: default_color,
            rng: rand::rng(),
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true; // We need the surface to be configured before we can do anything with it. We set the is_surface_configured flag to true here and we'll check it in the render() function
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => self.pipeline_state = !self.pipeline_state,
            _ => {
                self.camera_controller.handle_key(code, is_pressed);
            }
        }
    }

    pub fn handle_mouse_input(&mut self, device_id: DeviceId, state: ElementState, button: MouseButton) {
        println!("Device id: {:?}", device_id);
        println!("State: {:?}", state);
        println!("Button: {:?}", button);

        match state {
            ElementState::Pressed => {
                self.bckg_color = wgpu::Color { r: (self.rng.random_range(0.0..1.0)), g: (self.rng.random_range(0.0..1.0)), b: (self.rng.random_range(0.0..1.0)), a: (1.0) };
            }
            _ => {}
        }

    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;

        let texture_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default()); // creates a TextureView with default settings. We need to do this because we want to control how the render code interacts with the texture

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.bckg_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        if !self.pipeline_state {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            /*
                The first is what buffer slot to use for this vertex buffer. You can have multiple vertex buffers set at a time

                The second parameter is the slice of the buffer to use.
                You can store as many objects in a buffer as your hardware allows, so slice allows us to specify which portion of the buffer to use.
                We use .. to specify the entire buffer.
            */
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        } else {
            render_pass.set_pipeline(&self.second_render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        drop(render_pass); // used to drop the reference of the encoder so we can call the finish method from the encoder

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish())); // tells wgpu to finish the command buffer and submit it to the GPU's render queue.
        output.present();

        Ok(())
    }
}