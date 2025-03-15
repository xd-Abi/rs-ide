extern crate gl;
mod application;
mod events;
mod platform;

use crate::application::Application;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let mut app = Application::new();
    app.init();
    app.run();
}
