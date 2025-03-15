#[derive(Debug, Clone)]
pub enum Event {
    WindowClose,
    WindowResized(u32, u32),
    MouseMove(i32, i32),
    MouseDown(MouseButton),
    MouseRelease(MouseButton)
}

#[derive(Debug, Clone)]
pub enum MouseButton {
    Right,
    Middle,
    Left,
}