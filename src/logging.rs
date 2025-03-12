use std::panic;
use std::panic::PanicHookInfo;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use tracing::{debug, error, info, warn};

pub fn init() {
    let tracing_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("Failed to set default tracing subscriber");

    panic::set_hook(Box::new(|info| panic_hook(info)));
    info!("Logging initialized");
}

fn panic_hook(panic_info: &PanicHookInfo) {
    let payload = panic_info.payload();

    let message = if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic payload".to_string()
    };

    let location = panic_info
        .location()
        .map(|l| l.to_string())
        .unwrap_or_else(|| "Unknown location".to_string());

    error!(
        location = %location,
        "{}", message
    );
}
