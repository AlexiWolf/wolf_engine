use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;
use crate::{ButtonState, Input, ToInput};

use winit::event::{KeyEvent, MouseScrollDelta, WindowEvent};
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton as WinitMouseButton, RawKeyEvent},
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
            WindowEvent::CursorMoved { position, .. } => Some(Input::MouseMove {
                x: position.x.trunc() as f32,
                y: position.y.trunc() as f32,
            }),
            WindowEvent::MouseInput { state, button, .. } => Some(Input::MouseButton {
                state: (*state).into(),
                button: (*button).into(),
            }),
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                ..
            } => Some(Input::MouseScroll {
                delta_x: *x,
                delta_y: *y,
            }),
            _ => None,
        }
    }
}

impl From<KeyEvent> for Input {
    fn from(event: KeyEvent) -> Input {
        let state = event.state.into();
        let scancode = event.physical_key.to_scancode().unwrap_or(0);
        let keycode = match event.physical_key.into() {
            KeyCode::Unknown => None,
            keycode => Some(keycode),
        };
        let is_repeat = event.repeat;
        Input::Keyboard {
            state,
            scancode,
            keycode,
            is_repeat,
        }
    }
}

impl ToInput for DeviceEvent {
    fn to_input(&self) -> Option<Input> {
        match self {
            DeviceEvent::Key(event) => Some(event.clone().into()),
            DeviceEvent::MouseMotion { delta } => Some(Input::RawMouseMove {
                delta_x: delta.0 as f32,
                delta_y: delta.1 as f32,
            }),
            _ => None,
        }
    }
}

impl From<RawKeyEvent> for Input {
    fn from(event: RawKeyEvent) -> Input {
        let state = event.state.into();
        let scancode = event.physical_key.to_scancode().unwrap_or(0);
        let keycode = match event.physical_key.into() {
            KeyCode::Unknown => None,
            keycode => Some(keycode),
        };
        Input::Keyboard {
            state,
            scancode,
            keycode,
            is_repeat: false,
        }
    }
}

impl From<ElementState> for ButtonState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => ButtonState::Down,
            ElementState::Released => ButtonState::Up,
        }
    }
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Right => MouseButton::Right,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Back => MouseButton::Back,
            WinitMouseButton::Forward => MouseButton::Forward,
            WinitMouseButton::Other(num) => MouseButton::Other(num as u32),
        }
    }
}

impl From<PhysicalKey> for KeyCode {
    fn from(key: PhysicalKey) -> KeyCode {
        match key {
            PhysicalKey::Code(keycode) => keycode.into(),
            PhysicalKey::Unidentified(_) => KeyCode::Unknown,
        }
    }
}

impl From<WinitKeyCode> for KeyCode {
    fn from(keycode: WinitKeyCode) -> Self {
        match keycode {
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
