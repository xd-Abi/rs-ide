mod application;
mod platform;

use tracing_subscriber;
use crate::application::Application;

fn main() {
    tracing_subscriber::fmt::init();
    let app = Application::new();
    app.run();
}
