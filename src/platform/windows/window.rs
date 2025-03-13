use crate::platform::windows::common::get_instance_handle;
use crate::{pcstr, static_pcstr};
use std::sync::{Mutex, OnceLock};
use tracing::info;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, PeekMessageA,
    PostQuitMessage, RegisterClassA, TranslateMessage, UnregisterClassA, CW_USEDEFAULT, MSG,
    PM_REMOVE, WINDOW_EX_STYLE, WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

#[derive(Debug)]
pub struct Window {
    handle: HWND,
    width: u32,
    height: u32,
}

static CLASS_NAME: &[u8] = b"RustIdeWindow\0";
static WINDOW_COUNT: OnceLock<Mutex<u8>> = OnceLock::new();

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let instance = get_instance_handle();
        let mut window_count = WINDOW_COUNT
            .get_or_init(|| {
                register_window_class(instance);
                Mutex::new(0)
            })
            .lock()
            .expect("Failed to lock window count mutex");

        unsafe {
            info!(title = %title, width = %width, height = %height, "Creating window...");
            let handle = CreateWindowExA(
                WINDOW_EX_STYLE(0),
                static_pcstr!(CLASS_NAME),
                pcstr!(title),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                Some(instance),
                None,
            )
            .expect("Failed to create window");

            *window_count += 1;
            Window {
                handle,
                width,
                height,
            }
        }
    }

    pub fn update(&self) {
        unsafe {
            let mut msg = MSG::default();
            while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let mut window_count = WINDOW_COUNT
            .get()
            .expect("Failed to get window count mutex")
            .lock()
            .expect("Failed to lock window count mutex");

        *window_count -= 1;
        let instance = get_instance_handle();

        unsafe {
            info!("Destroying window...");
            DestroyWindow(self.handle).expect("Failed to destroy window");

            if *window_count == 0 {
                info!("Unregistering window class...");
                UnregisterClassA(static_pcstr!(CLASS_NAME), Some(instance))
                    .expect("Failed to unregister class");
            }
        }
    }
}

fn register_window_class(instance: HINSTANCE) {
    info!("Registering window class...");

    unsafe {
        let wnd_class = WNDCLASSA {
            lpszClassName: static_pcstr!(CLASS_NAME),
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
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

unsafe extern "system" fn window_proc(
    handle: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        _ => DefWindowProcA(handle, msg, w_param, l_param),
    }
}
