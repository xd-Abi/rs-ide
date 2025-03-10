pub trait WindowTrait {
    fn new(title: &str, width: u32, height: u32) -> Self;
    fn update(&self);
    fn is_close_requested(&self) -> bool;
}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::window::WindowsWindow as Window;
