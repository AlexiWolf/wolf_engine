use crate::keyboard::{Key, KeyCode};
use crate::{Input, InputConversion};

use winit::{
    event::{DeviceEvent, ElementState, Event, RawKeyEvent},
    keyboard::{KeyCode as WinitKeyCode, PhysicalKey},
    platform::scancode::PhysicalKeyExtScancode,
};

impl<T> InputConversion for winit::event::Event<T> {
    fn as_input(&self) -> Option<Input> {
        match self {
            Event::DeviceEvent {
                event: DeviceEvent::Key(key_event),
                ..
            } => Some(key_event.clone().into()),
            _ => None,
        }
    }
}

impl Into<Input> for RawKeyEvent {
    fn into(self) -> Input {
        match self.state {
            ElementState::Pressed => Input::KeyDown(self.physical_key.into()),
            ElementState::Released => Input::KeyUp(self.physical_key.into()),
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
            WinitKeyCode::KeyA => KeyCode::A,
            WinitKeyCode::KeyB => KeyCode::B,
            WinitKeyCode::KeyC => KeyCode::C,
            WinitKeyCode::KeyD => KeyCode::D,
            WinitKeyCode::KeyE => KeyCode::E,
            WinitKeyCode::KeyF => KeyCode::F,
            WinitKeyCode::KeyG => KeyCode::G,
            WinitKeyCode::KeyH => KeyCode::H,
            WinitKeyCode::KeyI => KeyCode::I,
            WinitKeyCode::KeyJ => KeyCode::J,
            WinitKeyCode::KeyK => KeyCode::K,
            WinitKeyCode::KeyL => KeyCode::L,
            WinitKeyCode::KeyM => KeyCode::M,
            WinitKeyCode::KeyN => KeyCode::N,
            WinitKeyCode::KeyO => KeyCode::O,
            WinitKeyCode::KeyP => KeyCode::P,
            WinitKeyCode::KeyQ => KeyCode::Q,
            WinitKeyCode::KeyR => KeyCode::R,
            WinitKeyCode::KeyS => KeyCode::S,
            WinitKeyCode::KeyT => KeyCode::T,
            WinitKeyCode::KeyU => KeyCode::U,
            WinitKeyCode::KeyV => KeyCode::V,
            WinitKeyCode::KeyW => KeyCode::W,
            WinitKeyCode::KeyX => KeyCode::X,
            WinitKeyCode::KeyY => KeyCode::Y,
            WinitKeyCode::KeyZ => KeyCode::Z,
            WinitKeyCode::Digit0 => KeyCode::Num0,
            WinitKeyCode::Digit1 => KeyCode::Num1,
            WinitKeyCode::Digit2 => KeyCode::Num2,
            WinitKeyCode::Digit3 => KeyCode::Num3,
            WinitKeyCode::Digit4 => KeyCode::Num4,
            WinitKeyCode::Digit5 => KeyCode::Num5,
            WinitKeyCode::Digit6 => KeyCode::Num6,
            WinitKeyCode::Digit7 => KeyCode::Num7,
            WinitKeyCode::Digit8 => KeyCode::Num8,
            WinitKeyCode::Digit9 => KeyCode::Num9,
            _ => KeyCode::Unknown,
        }
    }
}

#[cfg(test)]
mod winit_conversion_tests {
    use ::winit::{
        event::{DeviceEvent, DeviceId, ElementState, Event, RawKeyEvent},
        keyboard::{KeyCode as WinitKeyCode, PhysicalKey},
    };
    use test_case::test_case;

    use super::*;
    use crate::*;

    #[test_case(WinitKeyCode::KeyA, ElementState::Pressed, Some(KeyCode::A))]
    #[test_case(WinitKeyCode::KeyA, ElementState::Released, Some(KeyCode::A))]
    fn should_convert_key_events(
        key_code: WinitKeyCode,
        state: ElementState,
        expected_keycode: Option<KeyCode>,
    ) {
        let event = create_winit_event(key_code, state);
        let input = event.as_input().expect("Input was not converted");
        match input {
            Input::KeyDown(key) => {
                assert!(
                    state.is_pressed(),
                    "The key was not pressed, so this should be a KeyUp event"
                );
                assert_ne!(key.scancode, 0, "The converted scancode should not be 0");
                assert_eq!(
                    key.keycode, expected_keycode,
                    "The keycode did not match what was expected"
                );
            }
            Input::KeyUp(key) => {
                assert!(
                    !state.is_pressed(),
                    "The key was pressed, so this should be a KeyDown event"
                );
                assert_ne!(key.scancode, 0, "The converted scancode should not be 0");
                assert_eq!(
                    key.keycode, expected_keycode,
                    "The keycode did not match what was expected"
                );
            }
        }
    }

    fn create_winit_event(key_code: WinitKeyCode, state: ElementState) -> winit::Event<()> {
        let raw_key_event = RawKeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state,
        };
        Event::DeviceEvent::<()> {
            // SAFETY: Don't pass this to any winit functions.
            device_id: unsafe { DeviceId::dummy() },
            event: DeviceEvent::Key(raw_key_event),
        }
    }
}
