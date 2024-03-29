//! Provides types related to mouse input.

/// Identifies a mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Forward,
    Back,
    Other(u32),
}
