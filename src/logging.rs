use tracing::Level;
use tracing_subscriber::FmtSubscriber;
pub use tracing::{error, warn, info, debug, trace};

pub fn init() {
    let tracing_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("Failed to set default tracing subscriber");
}

