//! Provides types related to keyboard input.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Key {
    pub scancode: u32,
    pub keycode: Option<KeyCode>,
}

/// Provides a set of named keys based on a US 104-key QWERTY keyboard.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum KeyCode {
    Escape,
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
    PrintScreen,
    ScrollLock,
    Pause,

    Grave,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,
    BackSlash,
    Backspace,
    Insert,
    Home,
    PageUp,
    NumLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,

    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBracket,
    RightBracket,
    Delete,
    End,
    PageDown,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,

    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Enter,
    Numpad4,
    Numpad5,
    Numpad6,

    LeftShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    ForwardSlash,
    RightShift,
    UpArrow,
    Numpad1,
    Numpad2,
    Numpad3,
    NumpadEnter,

    LeftControl,
    LeftSuper,
    LeftAlt,
    Space,
    RightAlt,
    RightSuper,
    RightControl,
    LeftArrow,
    DownArrow,
    RightArrow,
    Numpad0,
    NumpadDecimal,

    Unknown,
}
