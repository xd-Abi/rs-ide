#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    WindowClose,
    WindowResized(u32, u32),
    MouseMove(i32, i32),
    MouseDown(MouseButton),
    MouseRelease(MouseButton),
    KeyDown(Key),
    KeyRelease(Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
    Extra5,
    Extra6,
    Extra7,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number Keys 0-9
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Special Keys
    Escape,
    Tab,
    CapsLock,
    Shift,
    Ctrl,
    LeftSuper,
    RightSuper,
    Alt,
    Space,
    Enter,
    Backspace,

    // Function Keys (F1 - F12)
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Arrow Keys
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
}
