use std::sync::Arc;

mod state;
mod camera;
mod camera_controller;
mod texture;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{
        ActiveEventLoop,
        EventLoop
    },
    keyboard::{
        PhysicalKey
    },
    window::Window
};

pub struct App {
    /*
        The state variable stores our State struct as an option.
        The reason we need an option is that State::new() needs a window and we can't create a window until the application gets to the Resumed state.
    */
    state: Option<state::State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<state::State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // If we are not on web we can use pollster to
        // await the
        self.state = Some(pollster::block_on(state::State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: state::State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let my_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => my_state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                my_state.update();
                match my_state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = my_state.window.inner_size();
                        my_state.resize(size.width, size.height);
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
            } => {
                my_state.handle_key(event_loop, code, key_state.is_pressed())
            }
            WindowEvent::MouseInput { device_id, state, button } => {
                my_state.handle_mouse_input(device_id, state, button)
            }
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
    let _ = run();
}
