use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::state::State;

#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        self.window = Some(window.clone());

        let state = tokio_runtime.block_on(async { return State::new(window.clone()).await });
        self.state = Some(state);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested if window_id == state.window().id() => {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(physycal_size) => {
                state.resize(physycal_size);
            }
            _ => (),
        }
    }
}
