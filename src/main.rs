use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{self, *},
    event_loop::{
        self, ActiveEventLoop, EventLoop
    },
    keyboard::{
        KeyCode,
        PhysicalKey
    },
    window::Window
};

// This will store the state of our game
pub struct State {
    window: Arc<Window>, // Arc is equivalent to a Shared ptr atomic
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
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

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
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

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // remove `todo!()`
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

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        } // used to drop the reference of the encoder so we can call the finish method from the encoder

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish())); // tells wgpu to finish the command buffer and submit it to the GPU's render queue.
        output.present();

        Ok(())
    }
}

pub struct App {
    /*
        The state variable stores our State struct as an option.
        The reason we need an option is that State::new() needs a window and we can't create a window until the application gets to the Resumed state.
    */
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // If we are not on web we can use pollster to
        // await the
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    let _ = run();
}
