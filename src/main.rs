use crate::logging::warn;
use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

mod config;
mod logging;
mod platform;

fn to_pcstr(s: &str) -> CString {
    CString::new(s).unwrap()  // Returns CString, keeping it alive!
}

fn main() {
    logging::init();
    let config = config::load().unwrap_or_else(|err| {
        warn!(error = %err, "Failed to load configuration");

        let config = config::Config::default();
        config::save(&config).expect("Failed to save configuration");
        config
    });

    unsafe {
        MessageBoxA(
            None,
            pcstr!("Hello, Welcome to Rust"),
            pcstr!("Rust IDE"),
            MB_OK | MB_ICONERROR,
        );
    };
}
