use crate::events::{Event, EventQueue, Key};
use crate::platform::windows::window;
use crate::platform::PlatformWindow;
use crate::{pcstr, static_pcstr};
use std::sync::{Arc, Mutex, OnceLock};
use tracing::{error, info};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetSysColorBrush, COLOR_WINDOW};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, GetWindowLongPtrA,
    PeekMessageA, RegisterClassA, SetWindowLongPtrA, TranslateMessage, UnregisterClassA,
    CREATESTRUCTA, CW_USEDEFAULT, GWLP_USERDATA, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_CLOSE,
    WM_CREATE, WM_NCDESTROY, WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

/// Represents a platform-specific native window on Windows.
///
/// This struct wraps a Win32 window handle and provides integration with
/// the application's event system via `EventQueue`. It internally manages
/// window creation, destruction, and event dispatch.
#[derive(Debug)]
pub struct WindowsWindow {
    event_queue: Arc<EventQueue>,
    handle: HWND,
}

/// The default class name for window registration.
///
/// This name is used when registering a window class with the Windows API.
pub const WINDOW_CLASS_NAME: &[u8] = b"RustIdeWindow\0";

/// Tracks the number of windows currently alive in this process.
///
/// Used to register/unregister the window class when this is zero.
static WINDOW_COUNT: OnceLock<Mutex<u8>> = OnceLock::new();

impl PlatformWindow for WindowsWindow {
    fn new(title: &str, event_queue: Arc<EventQueue>) -> Self {
        let instance = get_instance_handle();
        let mut window_count = WINDOW_COUNT
            .get_or_init(|| {
                register_window_class(instance);
                Mutex::new(0)
            })
            .lock()
            .expect("Failed to lock window count mutex");

        unsafe {
            let event_queue_ptr = Arc::into_raw(event_queue.clone());

            info!(title = %title, "Creating window...");
            let handle = CreateWindowExA(
                WINDOW_EX_STYLE(0),
                static_pcstr!(WINDOW_CLASS_NAME),
                pcstr!(title),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Some(instance),
                Some(event_queue_ptr as *mut std::ffi::c_void),
            )
            .expect("Failed to create window");

            *window_count += 1;

            WindowsWindow {
                event_queue,
                handle,
            }
        }
    }

    fn update(&self) {
        unsafe {
            let mut msg = MSG::default();
            // Process one message per frame (avoids blocking)
            if PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}

impl Drop for WindowsWindow {
    /// Cleans up the window.
    ///
    /// Destroys the `HWND`, and if it was the last window, unregisters the class.
    fn drop(&mut self) {
        info!("Destroying window...");
        let instance = get_instance_handle();
        let mut window_count = WINDOW_COUNT
            .get()
            .expect("Failed to get window count mutex")
            .lock()
            .expect("Failed to lock window count mutex");

        unsafe {
            DestroyWindow(self.handle).expect("Failed to destroy window");
            *window_count -= 1;

            if *window_count == 0 {
                unregister_window_class(instance);
            }
        }
    }
}

/// Registers the Win32 window class (only once).
///
/// This must be done before any windows are created.
///
/// # Arguments
/// * `instance` - The HINSTANCE used for registration.
fn register_window_class(instance: HINSTANCE) {
    info!("Registering window class...");

    unsafe {
        let wnd_class = WNDCLASSA {
            lpszClassName: static_pcstr!(WINDOW_CLASS_NAME),
            lpfnWndProc: Some(window_proc),
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

/// Unregisters the Win32 window class after all windows are gone.
///
/// Should only be called after window count hits zero.
///
/// # Arguments
/// * `instance` - The same HINSTANCE used to register the class.
fn unregister_window_class(instance: HINSTANCE) {
    info!("Unregistering window class...");
    unsafe {
        // Drain any remaining messages (e.g. WM_NCDESTROY)
        let mut msg = MSG::default();
        while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnregisterClassA(static_pcstr!(WINDOW_CLASS_NAME), Some(instance))
            .expect("Failed to unregister class");
    }
}

/// Returns the module handle for the current binary.
///
/// Used for window creation and class registration.
///
/// # Returns
/// * `HINSTANCE` - Module handle of the running process.
fn get_instance_handle() -> HINSTANCE {
    unsafe {
        GetModuleHandleA(None)
            .expect("Failed to get module handle.")
            .into()
    }
}

/// Extracts the `Arc<EventQueue>` from a window's user data.
///
/// This macro retrieves the pointer previously stored via `SetWindowLongPtrA`
/// (during `WM_CREATE`) and converts it back to an `Arc<EventQueue>`.
///
/// If the pointer is null, the macro logs an error and returns early
/// with a call to `DefWindowProcA`.
///
/// Internally, this macro performs a temporary `Arc::from_raw` followed
/// by `clone()` and `forget()` to avoid affecting the reference count.
///
/// # Arguments
/// * `$handle`  - The `HWND` associated with the window.
/// * `$msg`     - The message ID (used in case of fallback).
/// * `$w_param` - WPARAM from the window procedure.
/// * `$l_param` - LPARAM from the window procedure.
///
/// # Returns
/// * `Arc<EventQueue>` - A safely cloned handle to the event queue.
///
/// # Usage
/// ```rust
/// let event_queue = get_event_queue!(handle, msg, w_param, l_param);
/// event_queue.push(Event::WindowClose);
/// ```
macro_rules! get_event_queue {
    ($handle:expr, $msg:expr, $w_param:expr, $l_param:expr) => {{
        let ptr = GetWindowLongPtrA($handle, GWLP_USERDATA) as *const EventQueue;
        if ptr.is_null() {
            error!("Event Queue pointer is null");
            return DefWindowProcA($handle, $msg, $w_param, $l_param);
        }

        let temp = Arc::from_raw(ptr);
        let event_queue = temp.clone();
        std::mem::forget(temp);

        event_queue
    }};
}

/// Window procedure callback for all windows of this class.
///
/// Handles creation, destruction, and close events.
///
/// # Arguments
/// * `handle`  - Handle to the window receiving the message.
/// * `msg`     - The Win32 message ID (e.g., `WM_CREATE`, `WM_CLOSE`, etc).
/// * `w_param` - Message-specific parameter.
/// * `l_param` - Message-specific parameter.
///
/// # Returns
/// * `LRESULT` - Result to pass back to the Windows message dispatcher.
pub unsafe extern "system" fn window_proc(
    handle: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let create_struct = &*(l_param.0 as *const CREATESTRUCTA);
            let event_queue_ptr = create_struct.lpCreateParams as *mut Arc<EventQueue>;

            if !event_queue_ptr.is_null() {
                SetWindowLongPtrA(handle, GWLP_USERDATA, event_queue_ptr as isize);
            }

            LRESULT(0)
        }
        WM_NCDESTROY => {
            let ptr = GetWindowLongPtrA(handle, GWLP_USERDATA) as *mut Arc<EventQueue>;
            if !ptr.is_null() {
                drop(Arc::from_raw(ptr));
            }

            LRESULT(0)
        }
        WM_CLOSE => {
            let event_queue = get_event_queue!(handle, msg, w_param, l_param);
            event_queue.push(Event::WindowClose);
            LRESULT(0)
        }
        _ => DefWindowProcA(handle, msg, w_param, l_param),
    }
}
