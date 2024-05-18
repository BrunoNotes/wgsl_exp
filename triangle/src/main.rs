use std::io;

use winit::event_loop::EventLoop;

mod state;
mod window;

fn main() -> Result<(), io::Error> {
    env_logger::init();

    // TODO: Change error
    let event_loop = EventLoop::new().map_err(|err| {
        eprintln!("Error creating event_loop: {}", err);
        return io::ErrorKind::Other;
    })?;

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = window::App::default();
    event_loop.run_app(&mut app).map_err(|err|{
        eprintln!("Error error running event_loop app: {}", err);
        return io::ErrorKind::Other;
    })?;

    Ok(())
}
