pub mod keyboard;

#[cfg(feature = "winit")]
pub mod winit;

use keyboard::Key;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
}
