use crate::keyboard::{Key, KeyCode};
use crate::{Input, ToInput};

use winit::event::{KeyEvent, WindowEvent};
use winit::{
    event::{DeviceEvent, ElementState, Event, RawKeyEvent},
    keyboard::{KeyCode as WinitKeyCode, PhysicalKey},
    platform::scancode::PhysicalKeyExtScancode,
};

impl<T> ToInput for winit::event::Event<T> {
    fn to_input(&self) -> Option<Input> {
        match self {
            Event::DeviceEvent { event, .. } => event.to_input(),
            Event::WindowEvent { event, .. } => event.to_input(),
            _ => None,
        }
    }
}

impl ToInput for WindowEvent {
    fn to_input(&self) -> Option<Input> {
        match self {
            WindowEvent::KeyboardInput { event, .. } => Some(event.clone().into()),
            _ => None,
        }
    }
}

impl Into<Input> for KeyEvent {
    fn into(self) -> Input {
        match self.state {
            ElementState::Pressed => Input::KeyDown {
                key: self.physical_key.into(),
            },
            ElementState::Released => Input::KeyUp {
                key: self.physical_key.into(),
            },
        }
    }
}

impl ToInput for DeviceEvent {
    fn to_input(&self) -> Option<Input> {
        match self {
            DeviceEvent::Key(event) => Some(event.clone().into()),
            _ => None,
        }
    }
}

impl Into<Input> for RawKeyEvent {
    fn into(self) -> Input {
        match self.state {
            ElementState::Pressed => Input::RawKeyDown {
                key: self.physical_key.into(),
            },
            ElementState::Released => Input::RawKeyUp {
                key: self.physical_key.into(),
            },
        }
    }
}

impl Into<Key> for PhysicalKey {
    fn into(self) -> Key {
        let scancode = self.to_scancode().unwrap_or(0);
        match self {
            PhysicalKey::Code(keycode) => Key {
                scancode,
                keycode: Some(keycode.into()),
            },
            PhysicalKey::Unidentified(_) => Key {
                scancode,
                keycode: None,
            },
        }
    }
}

impl Into<KeyCode> for WinitKeyCode {
    fn into(self) -> KeyCode {
        match self {
            WinitKeyCode::Escape => KeyCode::Escape,
            WinitKeyCode::F1 => KeyCode::F1,
            WinitKeyCode::F2 => KeyCode::F2,
            WinitKeyCode::F3 => KeyCode::F3,
            WinitKeyCode::F4 => KeyCode::F4,
            WinitKeyCode::F5 => KeyCode::F5,
            WinitKeyCode::F6 => KeyCode::F6,
            WinitKeyCode::F7 => KeyCode::F7,
            WinitKeyCode::F8 => KeyCode::F8,
            WinitKeyCode::F9 => KeyCode::F9,
            WinitKeyCode::F10 => KeyCode::F10,
            WinitKeyCode::F11 => KeyCode::F11,
            WinitKeyCode::F12 => KeyCode::F12,
            WinitKeyCode::PrintScreen => KeyCode::PrintScreen,
            WinitKeyCode::ScrollLock => KeyCode::ScrollLock,
            WinitKeyCode::Pause => KeyCode::Pause,

            WinitKeyCode::Backquote => KeyCode::Grave,
            WinitKeyCode::Digit1 => KeyCode::Num1,
            WinitKeyCode::Digit2 => KeyCode::Num2,
            WinitKeyCode::Digit3 => KeyCode::Num3,
            WinitKeyCode::Digit4 => KeyCode::Num4,
            WinitKeyCode::Digit5 => KeyCode::Num5,
            WinitKeyCode::Digit6 => KeyCode::Num6,
            WinitKeyCode::Digit7 => KeyCode::Num7,
            WinitKeyCode::Digit8 => KeyCode::Num8,
            WinitKeyCode::Digit9 => KeyCode::Num9,
            WinitKeyCode::Digit0 => KeyCode::Num0,
            WinitKeyCode::Minus => KeyCode::Minus,
            WinitKeyCode::Equal => KeyCode::Equals,
            WinitKeyCode::Backslash => KeyCode::BackSlash,
            WinitKeyCode::Backspace => KeyCode::Backspace,
            WinitKeyCode::Insert => KeyCode::Insert,
            WinitKeyCode::Home => KeyCode::Home,
            WinitKeyCode::PageUp => KeyCode::PageUp,
            WinitKeyCode::NumLock => KeyCode::NumLock,
            WinitKeyCode::NumpadDivide => KeyCode::NumpadDivide,
            WinitKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
            WinitKeyCode::NumpadSubtract => KeyCode::Backspace,

            WinitKeyCode::Tab => KeyCode::Tab,
            WinitKeyCode::KeyQ => KeyCode::Q,
            WinitKeyCode::KeyW => KeyCode::W,
            WinitKeyCode::KeyE => KeyCode::E,
            WinitKeyCode::KeyR => KeyCode::R,
            WinitKeyCode::KeyT => KeyCode::T,
            WinitKeyCode::KeyY => KeyCode::Y,
            WinitKeyCode::KeyU => KeyCode::U,
            WinitKeyCode::KeyI => KeyCode::I,
            WinitKeyCode::KeyO => KeyCode::O,
            WinitKeyCode::KeyP => KeyCode::P,
            WinitKeyCode::BracketLeft => KeyCode::LeftBracket,
            WinitKeyCode::BracketRight => KeyCode::RightBracket,
            WinitKeyCode::Delete => KeyCode::Delete,
            WinitKeyCode::End => KeyCode::End,
            WinitKeyCode::PageDown => KeyCode::PageDown,
            WinitKeyCode::Numpad7 => KeyCode::Numpad7,
            WinitKeyCode::Numpad8 => KeyCode::Numpad8,
            WinitKeyCode::Numpad9 => KeyCode::Numpad9,

            WinitKeyCode::CapsLock => KeyCode::CapsLock,
            WinitKeyCode::KeyA => KeyCode::A,
            WinitKeyCode::KeyS => KeyCode::S,
            WinitKeyCode::KeyD => KeyCode::D,
            WinitKeyCode::KeyF => KeyCode::F,
            WinitKeyCode::KeyG => KeyCode::G,
            WinitKeyCode::KeyH => KeyCode::H,
            WinitKeyCode::KeyJ => KeyCode::J,
            WinitKeyCode::KeyK => KeyCode::K,
            WinitKeyCode::KeyL => KeyCode::L,
            WinitKeyCode::Semicolon => KeyCode::Semicolon,
            WinitKeyCode::Quote => KeyCode::Quote,
            WinitKeyCode::Enter => KeyCode::Enter,
            WinitKeyCode::Numpad4 => KeyCode::Numpad4,
            WinitKeyCode::Numpad5 => KeyCode::Numpad5,
            WinitKeyCode::Numpad6 => KeyCode::Numpad6,

            WinitKeyCode::ShiftLeft => KeyCode::RightShift,
            WinitKeyCode::KeyZ => KeyCode::Z,
            WinitKeyCode::KeyX => KeyCode::X,
            WinitKeyCode::KeyC => KeyCode::C,
            WinitKeyCode::KeyV => KeyCode::V,
            WinitKeyCode::KeyB => KeyCode::B,
            WinitKeyCode::KeyN => KeyCode::N,
            WinitKeyCode::KeyM => KeyCode::M,
            WinitKeyCode::Comma => KeyCode::Comma,
            WinitKeyCode::Period => KeyCode::Period,
            WinitKeyCode::Slash => KeyCode::ForwardSlash,
            WinitKeyCode::ShiftRight => KeyCode::RightShift,
            WinitKeyCode::ArrowUp => KeyCode::UpArrow,
            WinitKeyCode::Numpad1 => KeyCode::Numpad1,
            WinitKeyCode::Numpad2 => KeyCode::Numpad2,
            WinitKeyCode::Numpad3 => KeyCode::Numpad3,
            WinitKeyCode::NumpadEnter => KeyCode::NumpadEnter,

            WinitKeyCode::ControlLeft => KeyCode::LeftControl,
            WinitKeyCode::SuperLeft => KeyCode::LeftSuper,
            WinitKeyCode::AltLeft => KeyCode::LeftAlt,
            WinitKeyCode::Space => KeyCode::Space,
            WinitKeyCode::AltRight => KeyCode::RightAlt,
            WinitKeyCode::SuperRight => KeyCode::RightSuper,
            WinitKeyCode::ControlRight => KeyCode::RightControl,
            WinitKeyCode::ArrowLeft => KeyCode::RightArrow,
            WinitKeyCode::ArrowDown => KeyCode::DownArrow,
            WinitKeyCode::ArrowRight => KeyCode::RightArrow,
            WinitKeyCode::Numpad0 => KeyCode::Numpad0,
            WinitKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,

            _ => KeyCode::Unknown,
        }
    }
}
