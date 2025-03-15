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
#[repr(u16)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    XButton1 = 3,
    XButton2 = 4,
    Extra5 = 5,
    Extra6 = 6,
    Extra7 = 7,
}

impl TryFrom<u16> for MouseButton {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MouseButton::Left),
            1 => Ok(MouseButton::Right),
            2 => Ok(MouseButton::Middle),
            3 => Ok(MouseButton::XButton1),
            4 => Ok(MouseButton::XButton2),
            5 => Ok(MouseButton::Extra5),
            6 => Ok(MouseButton::Extra6),
            7 => Ok(MouseButton::Extra7),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    Space = 32,
    Apostrophe = 39, /* ' */
    Comma = 44,      /* , */
    Minus = 45,      /* - */
    Period = 46,     /* . */
    Slash = 47,      /* / */

    D0 = 48, /* 0 */
    D1 = 49, /* 1 */
    D2 = 50, /* 2 */
    D3 = 51, /* 3 */
    D4 = 52, /* 4 */
    D5 = 53, /* 5 */
    D6 = 54, /* 6 */
    D7 = 55, /* 7 */
    D8 = 56, /* 8 */
    D9 = 57, /* 9 */

    Semicolon = 59, /* ; */
    Equal = 61,     /* = */

    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,

    LeftBracket = 91,  /* [ */
    Backslash = 92,    /* \ */
    RightBracket = 93, /* ] */
    GraveAccent = 96,  /* ` */

    World1 = 161, /* non-US #1 */
    World2 = 162, /* non-US #2 */

    /* Function keys */
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,

    /* Keypad */
    KP0 = 320,
    KP1 = 321,
    KP2 = 322,
    KP3 = 323,
    KP4 = 324,
    KP5 = 325,
    KP6 = 326,
    KP7 = 327,
    KP8 = 328,
    KP9 = 329,
    KPDecimal = 330,
    KPDivide = 331,
    KPMultiply = 332,
    KPSubtract = 333,
    KPAdd = 334,
    KPEnter = 335,
    KPEqual = 336,

    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}

impl TryFrom<u16> for Key {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            32 => Ok(Key::Space),
            39 => Ok(Key::Apostrophe),
            44 => Ok(Key::Comma),
            45 => Ok(Key::Minus),
            46 => Ok(Key::Period),
            47 => Ok(Key::Slash),

            48..=57 => Ok(unsafe { std::mem::transmute(value) }),
            59 => Ok(Key::Semicolon),
            61 => Ok(Key::Equal),
            65..=90 => Ok(unsafe { std::mem::transmute(value) }),

            91 => Ok(Key::LeftBracket),
            92 => Ok(Key::Backslash),
            93 => Ok(Key::RightBracket),
            96 => Ok(Key::GraveAccent),

            161 => Ok(Key::World1),
            162 => Ok(Key::World2),

            256..=348 => Ok(unsafe { std::mem::transmute(value) }),

            _ => Err(()),
        }
    }
}