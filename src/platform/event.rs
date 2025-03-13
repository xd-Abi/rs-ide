#[derive(Debug, Clone)]
pub enum Event {
    WindowClose,
    WindowResized(u32, u32),
}