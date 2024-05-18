use winit::event_loop::EventLoop;

mod window;
mod state;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = window::App::default();
    event_loop.run_app(&mut app).unwrap();
}
