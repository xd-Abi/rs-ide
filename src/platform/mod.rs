use crate::events::EventQueue;
use cfg_if::cfg_if;
use std::fmt::Debug;
use std::sync::Arc;

/// A trait defining a platform-specific window abstraction.
///
/// This trait allows different operating systems (Windows, Linux, macOS)
/// to implement their own native window handling while maintaining a
/// consistent interface across platforms.
///
/// ## Required Implementations
/// Each platform must implement:
/// - **`new(title, event_queue) -> Self`** → Creates a new window.
/// - **`update(&self)`** → Processes window events (e.g., input, resizing).
///
/// ## Platform-Specific Implementations
/// This trait should be implemented separately for each platform:
/// - **Windows** (WinAPI)
/// - **Linux** (X11, Wayland)
/// - **macOS** (Cocoa)
pub trait PlatformWindow: Debug {
    /// Creates a new platform-specific window.
    ///
    /// # Arguments
    /// - `title` - The title of the window (displayed in the title bar).
    /// - `event_queue` - A shared event queue for passing events to the application.
    ///
    /// # Returns
    /// A new instance of the platform-specific window type.
    ///
    /// # Notes
    /// - This function is expected to **create and initialize** the window.
    /// - The returned window instance should be ready to display.
    fn new(title: &str, event_queue: Arc<EventQueue>) -> Self
    where
        Self: Sized;

    /// Updates the window by processing platform-specific events.
    ///
    /// This function should be called every frame (e.g., inside an event loop)
    /// to handle window-related events, such as:
    /// - User input (mouse, keyboard)
    /// - Window resizing
    /// - Window close requests
    ///
    /// # Notes
    /// - Implementations **must** fetch and process platform-specific messages.
    /// - This function should be **non-blocking** to ensure smooth updates.
    fn update(&self);
}

#[cfg(target_os = "windows")]
mod windows;
cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub type Window = windows::window::WindowsWindow;
    }
}
