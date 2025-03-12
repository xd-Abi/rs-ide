mod application;
mod config;
mod logging;
mod window;
mod renderer;

use crate::application::App;
use crate::logging::warn;
use winit::event_loop::EventLoop;

fn main() {
    logging::init();
    let config = config::load().unwrap_or_else(|err| {
        warn!(error = %err, "Failed to load configuration");

        let config = config::Config::default();
        config::save(&config).expect("Failed to save configuration");
        config
    });

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App::new(config);

    event_loop
        .run_app(&mut app)
        .expect("Failed to run event loop");
}
