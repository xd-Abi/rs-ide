pub mod app;
mod events;

use app::App;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let mut app = App::new();
    app.run();
}
