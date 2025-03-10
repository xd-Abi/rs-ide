use crate::platform::WindowTrait;

pub struct WindowsWindow {
    width: u32,
    height: u32,
}

impl WindowTrait for WindowsWindow {
    fn new(title: &str, width: u32, height: u32) -> Self {
        WindowsWindow { width, height }
    }

    fn update(&self) {}

    fn is_close_requested(&self) -> bool {
        true
    }
}
