use winit::event_loop::EventLoop;

mod state;
mod window;

fn main() -> Result<(), ()> {
    env_logger::init();

    // TODO: Change error
    let event_loop = EventLoop::new().map_err(|err| {
        eprintln!("ERROR: creating event_loop: {}", err);
    })?;

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = window::App::default();
    event_loop.run_app(&mut app).map_err(|err| {
        eprintln!("ERROR: error running event_loop app: {}", err);
    })?;

    Ok(())
}
