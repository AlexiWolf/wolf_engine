//! Provides types related to mouse input.

/// Represents a mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Forward,
    Back,
    Other(u32),
}
