//! Provides an input for Wolf Engine.
//!
//! The main job of this crate is to provide a single set of common input events to make
//! input-handling in games / other applications easier.
//!
//! Adaptors for commonly-used APIs, such as [winit](::winit), are provided, and can be
//! enabled through their respective feature flags.

pub mod keyboard;
pub mod mouse;

#[cfg(feature = "winit")]
mod winit;

use keyboard::KeyCode;
use mouse::MouseButton;

/// Provides a set of common input events.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Input {
    /// A keyboard button was pressed / released.
    Keyboard {
        state: ButtonState,
        scancode: u32,
        keycode: Option<KeyCode>,

        /// Indicates if this is a repeat key press.
        ///
        /// Some platforms repeatedly emit key-down events if a key is held down for a certain
        /// amount of time.   This value is `true` for these repeat events.
        is_repeat: bool,
    },

    /// A keyboard button was pressed / released.
    ///
    /// This event is emitted by the OS, and is not associated with a window.  It may be emitted
    /// alongside a normal [`Keyboard`](Input::Keyboard) event.  Some window systems may filter
    /// out raw events when the window is not in-focus.
    RawKeyboard {
        state: ButtonState,
        scancode: u32,
        keycode: Option<KeyCode>,
    },
    MouseMoved {
        x: f32,
        y: f32,
    },
    RawMouseMoved {
        delta_x: f32,
        delta_y: f32,
    },
    MouseButton {
        state: ButtonState,
        button: MouseButton,
    },
}

/// Indicates the current state of a button input.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Down,
    Up,
}

/// Provides an adapter to convert external input events to an [`Input`].
///
/// It's best to implement this trait for the "main" event type of another library, even if not
/// all the events are input-related.  Non-input events should be ignored, and `None` returned
/// for them.  The conversion does not consume the source events, and will not stop them from
/// being processed down-stream.
pub trait ToInput {
    /// Convert a reference to `self` to an [`Input`], if possible.
    fn to_input(&self) -> Option<Input>;
}
