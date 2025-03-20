use crate::platform::windows::common::{get_instance_handle, WINDOW_CLASS_NAME};
use crate::static_pcstr;
use tracing::info;
use windows::Win32::Graphics::Gdi::{GetSysColorBrush, COLOR_WINDOW};
use windows::Win32::UI::WindowsAndMessaging::{RegisterClassA, UnregisterClassA, WNDCLASSA};

/// Bootstraps the Windows platform by registering the window class.
///
/// This function should be called **once at application startup**.
/// It ensures that the necessary window class is registered before
/// creating any windows.
///
/// # Example Usage
/// ```
/// bootstrap();
/// ```
pub fn bootstrap() {
    info!("Bootstrapping windows platform...");
    register_window_class();
}

/// Shuts down the Windows platform by unregistering the window class.
///
/// This function should be called **once before application shutdown**.
/// It ensures that the registered window class is properly removed.
///
/// # Example Usage
/// ```
/// shutdown();
/// ```
pub fn shutdown() {
    info!("Shutting down windows platform...");
    unregister_window_class();
}

/// Registers a Windows window class for use in the application.
///
/// This function is required to set up **WinAPI**'s class-based window system.
/// It only needs to be called **once per process**, before creating any windows.
///
/// # Safety:
/// - Uses **unsafe WinAPI calls**.
/// - **Must be called before creating a window**.
/// - If registration fails, the function will **panic**.
fn register_window_class() {
    info!("Registering window class...");

    unsafe {
        let instance = get_instance_handle();
        let wnd_class = WNDCLASSA {
            lpszClassName: static_pcstr!(WINDOW_CLASS_NAME),
            //lpfnWndProc: Some(window_proc),
            hInstance: instance,
            hbrBackground: GetSysColorBrush(COLOR_WINDOW),
            ..Default::default()
        };

        if RegisterClassA(&wnd_class) == 0 {
            panic!(
                "Failed to register window class: {}",
                windows::core::Error::from_win32()
            );
        }
    }
}

/// Unregisters the previously registered window class.
///
/// This function should be called when the application is shutting down
/// to ensure that the registered class is properly cleaned up.
///
/// # Safety:
/// - Uses **unsafe WinAPI calls**.
/// - If the class was never registered, this function **may fail**.
/// - If an active window is still using this class, **this may cause issues**.
fn unregister_window_class() {
    info!("Unregistering window class...");
    unsafe {
        let instance = get_instance_handle();
        UnregisterClassA(static_pcstr!(WINDOW_CLASS_NAME), Some(instance))
            .expect("Failed to unregister class");
    }
}
