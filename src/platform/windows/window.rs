use crate::platform::WindowTrait;
use std::sync::atomic::{AtomicU32, Ordering};
use windows::core::PCSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct WindowsWindow {
    handle: HWND,
}

static CLASS_NAME: &str = "RustIDEWindow";
static WINDOW_COUNT: AtomicU32 = AtomicU32::new(0);

impl WindowTrait for WindowsWindow {
    fn new(title: &str, width: u32, height: u32) -> Self {
        unsafe {
            let instance: HINSTANCE = GetModuleHandleA(None)
                .expect("Failed to get module handle")
                .into();
            let window_count = WINDOW_COUNT.load(Ordering::SeqCst);
            let class_name = PCSTR(CLASS_NAME.as_ptr());

            if window_count == 0 {
                let wc = WNDCLASSA {
                    hInstance: instance,
                    lpszClassName: class_name,
                    lpfnWndProc: Some(window_proc),
                    ..Default::default()
                };

                RegisterClassA(&wc);
            }

            let handle = CreateWindowExA(
                Default::default(),
                class_name,
                PCSTR(title.as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                instance.into(),
                None,
            )
            .expect("Failed to create window");

            WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

            WindowsWindow { handle }
        }
    }

    fn update(&self) {
        unsafe {
            let mut msg = MSG::default();
            while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}

impl Drop for WindowsWindow {
    fn drop(&mut self) {
        println!("Dropped window");

        unsafe {
            DestroyWindow(self.handle).expect("Failed to destroy window");
        }

        let count = WINDOW_COUNT.fetch_sub(1, Ordering::SeqCst);
        if count == 0 {
            println!("DestroyWindow Class");

            unsafe {
                let instance: HINSTANCE = GetModuleHandleA(None)
                    .expect("Failed to get module handle")
                    .into();

                UnregisterClassA(PCSTR(CLASS_NAME.as_ptr()), instance.into())
                    .expect("Failed to unregister class handle");
            }
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    return match msg {
        WM_CLOSE => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    };
}
