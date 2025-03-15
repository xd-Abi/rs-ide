use crate::events::{Event, Key, MouseButton};
use crate::platform::windows::common::get_instance_handle;
use crate::{get_window_mut, get_x_lparam, get_y_lparam, hiword, loword, pcstr, static_pcstr};
use std::fmt;
use std::fmt::Debug;
use std::sync::{Mutex, OnceLock};
use tracing::{error, info};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetSysColorBrush, ScreenToClient, COLOR_WINDOW};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, Default)]
struct WindowHandle(HWND);

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}

impl Into<HWND> for WindowHandle {
    fn into(self) -> HWND {
        self.0
    }
}

type EventCallback = Box<dyn Fn(Event) + 'static>;

pub struct Window {
    handle: WindowHandle,
    width: u32,
    height: u32,
    event_callback: Option<EventCallback>,
}

const CLASS_NAME: &[u8] = b"RustIdeWindow\0";
const CAPTION_HEIGHT: i32 = 50;
const BORDER_THICKNESS: i32 = 7;
static WINDOW_COUNT: OnceLock<Mutex<u8>> = OnceLock::new();

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Box<Window> {
        let instance = get_instance_handle();
        let mut window_count = WINDOW_COUNT
            .get_or_init(|| {
                register_window_class(instance);
                Mutex::new(0)
            })
            .lock()
            .expect("Failed to lock window count mutex");

        let mut window = Box::new(Window {
            handle: WindowHandle::default(),
            width,
            height,
            event_callback: None,
        });

        unsafe {
            info!(title = %title, width = %width, height = %height, "Creating window...");
            let handle = CreateWindowExA(
                WINDOW_EX_STYLE(0),
                static_pcstr!(CLASS_NAME),
                pcstr!(title),
                WS_POPUP
                    | WS_THICKFRAME
                    | WS_SYSMENU
                    | WS_MAXIMIZEBOX
                    | WS_MINIMIZEBOX
                    | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                Some(instance),
                Some(window.as_mut() as *mut _ as _),
            )
            .expect("Failed to create window");

            *window_count += 1;
            window.handle = WindowHandle(handle);
        }

        window
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

    fn trigger_event(&self, event: Event) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    pub fn set_event_callback<F: Fn(Event) + 'static>(&mut self, callback: F) {
        self.event_callback = Some(Box::new(callback));
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
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
            DestroyWindow(self.handle.0).expect("Failed to destroy window");

            if *window_count == 0 {
                info!("Unregistering window class...");
                UnregisterClassA(static_pcstr!(CLASS_NAME), Some(instance))
                    .expect("Failed to unregister class");
            }
        }
    }
}

impl Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.handle)
            .field("width", &self.width)
            .field("height", &self.height)
            // Can't print closures, so just label it
            .field("event_callback", &"FnMut(...)")
            .finish()
    }
}

fn register_window_class(instance: HINSTANCE) {
    info!("Registering window class...");

    unsafe {
        let wnd_class = WNDCLASSA {
            lpszClassName: static_pcstr!(CLASS_NAME),
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

unsafe extern "system" fn window_proc(
    handle: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_NCCREATE => {
                let create_struct = &*(l_param.0 as *const CREATESTRUCTW);
                let window_ptr = create_struct.lpCreateParams as *mut Window;

                if !window_ptr.is_null() {
                    SetWindowLongPtrW(handle, GWLP_USERDATA, window_ptr as isize);
                }

                LRESULT(1)
            }
            WM_NCHITTEST => {
                let mut cursor_pos = POINT {
                    x: get_x_lparam!(l_param) as i32,
                    y: get_y_lparam!(l_param) as i32,
                };

                ScreenToClient(handle, &mut cursor_pos)
                    .expect("Failed to convert screen coordinates to client coordinates");

                let mut rect = RECT::default();
                GetClientRect(handle, &mut rect).expect("Failed to get client rect");

                let x = cursor_pos.x;
                let y = cursor_pos.y;

                const LEFT: i32 = 1;
                const TOP: i32 = 2;
                const RIGHT: i32 = 4;
                const BOTTOM: i32 = 8;

                let mut hit = 0;
                if x < BORDER_THICKNESS {
                    hit |= LEFT;
                }
                if x > rect.right - BORDER_THICKNESS {
                    hit |= RIGHT;
                }
                if y < BORDER_THICKNESS {
                    hit |= TOP;
                }
                if y > rect.bottom - BORDER_THICKNESS {
                    hit |= BOTTOM;
                }

                if (hit & TOP != 0) && (hit & LEFT != 0) {
                    return LRESULT(HTTOPLEFT as isize);
                }
                if (hit & TOP != 0) && (hit & RIGHT != 0) {
                    return LRESULT(HTTOPRIGHT as isize);
                }
                if (hit & BOTTOM != 0) && (hit & LEFT != 0) {
                    return LRESULT(HTBOTTOMLEFT as isize);
                }
                if (hit & BOTTOM != 0) && (hit & RIGHT != 0) {
                    return LRESULT(HTBOTTOMRIGHT as isize);
                }
                if hit & LEFT != 0 {
                    return LRESULT(HTLEFT as isize);
                }
                if hit & TOP != 0 {
                    return LRESULT(HTTOP as isize);
                }
                if hit & RIGHT != 0 {
                    return LRESULT(HTRIGHT as isize);
                }
                if hit & BOTTOM != 0 {
                    return LRESULT(HTBOTTOM as isize);
                }
                if y < rect.top + CAPTION_HEIGHT {
                    return LRESULT(HTCAPTION as isize);
                }

                SetCursor(Some(
                    LoadCursorW(None, IDC_ARROW).expect("Failed to load arrow cursor"),
                ));
                LRESULT(HTCLIENT as isize)
            }
            WM_CLOSE => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::WindowClose);
                LRESULT(0)
            }
            WM_SIZE => {
                let window = get_window_mut!(handle, msg, w_param, l_param);

                let new_width = loword!(l_param) as u32;
                let new_height = hiword!(l_param) as u32;

                window.width = new_width;
                window.height = new_height;
                window.trigger_event(Event::WindowResized(new_width, new_height));

                LRESULT(0)
            }
            WM_MOUSEMOVE => {
                let x = get_x_lparam!(l_param) as i32;
                let y = get_y_lparam!(l_param) as i32;
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::MouseMove(x, y));
                LRESULT(0)
            }
            WM_LBUTTONDOWN => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                SetCapture(handle);
                window.trigger_event(Event::MouseDown(MouseButton::Left));
                LRESULT(0)
            }
            WM_LBUTTONUP => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::MouseRelease(MouseButton::Left));
                ReleaseCapture();
                LRESULT(0)
            }
            WM_RBUTTONDOWN => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                SetCapture(handle);
                window.trigger_event(Event::MouseDown(MouseButton::Right));
                LRESULT(0)
            }
            WM_RBUTTONUP => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::MouseRelease(MouseButton::Right));
                ReleaseCapture();
                LRESULT(0)
            }
            WM_MBUTTONDOWN => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                SetCapture(handle);
                window.trigger_event(Event::MouseDown(MouseButton::Middle));
                LRESULT(0)
            }
            WM_MBUTTONUP => {
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::MouseRelease(MouseButton::Middle));
                ReleaseCapture();
                LRESULT(0)
            }
            WM_XBUTTONDOWN => {
                let button = match hiword!(w_param) {
                    1 => MouseButton::XButton1,
                    2 => MouseButton::XButton2,
                    _ => return DefWindowProcA(handle, msg, w_param, l_param),
                };
                let window = get_window_mut!(handle, msg, w_param, l_param);
                SetCapture(handle);
                window.trigger_event(Event::MouseDown(button));

                // Prevent further processing
                LRESULT(1)
            }
            WM_XBUTTONUP => {
                let button = match hiword!(w_param) {
                    1 => MouseButton::XButton1,
                    2 => MouseButton::XButton2,
                    _ => return DefWindowProcA(handle, msg, w_param, l_param),
                };
                let window = get_window_mut!(handle, msg, w_param, l_param);
                window.trigger_event(Event::MouseRelease(button));
                ReleaseCapture();

                // Prevent further processing
                LRESULT(1)
            }
            _ => DefWindowProcA(handle, msg, w_param, l_param),
        }
    }
}
