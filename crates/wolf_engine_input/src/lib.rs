pub mod keyboard;

#[cfg(feature = "winit")]
mod winit;

use keyboard::Key;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    KeyDown { key: Key, is_repeat: bool },
    KeyUp { key: Key },

    RawKeyDown { key: Key },
    RawKeyUp { key: Key },
}

pub trait ToInput {
    fn to_input(&self) -> Option<Input>;
}
