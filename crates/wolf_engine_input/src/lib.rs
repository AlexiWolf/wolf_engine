#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Key {
    pub scancode: u32,
    pub keycode: Option<KeyCode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
}

#[cfg(feature = "winit")]
pub mod winit {
    use crate::{Input, Key, KeyCode};
    use winit::{
        event::{DeviceEvent, ElementState, Event, RawKeyEvent},
        keyboard::PhysicalKey,
        platform::scancode::PhysicalKeyExtScancode,
    };

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

    impl Into<KeyCode> for winit::keyboard::KeyCode {
        fn into(self) -> KeyCode {
            match self {
                _ => todo!(),
            }
        }
    }

    pub fn winit_to_input<UserEvent>(event: Event<UserEvent>) -> Option<Input> {
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::Key(key_event),
                ..
            } => Some(key_event.into()),
            _ => None,
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
        #[test_case(WinitKeyCode::KeyB, ElementState::Pressed, Some(KeyCode::B))]
        #[test_case(WinitKeyCode::KeyC, ElementState::Pressed, Some(KeyCode::C))]
        #[test_case(WinitKeyCode::KeyD, ElementState::Pressed, Some(KeyCode::D))]
        #[test_case(WinitKeyCode::KeyE, ElementState::Pressed, Some(KeyCode::E))]
        #[test_case(WinitKeyCode::KeyF, ElementState::Pressed, Some(KeyCode::F))]
        #[test_case(WinitKeyCode::KeyG, ElementState::Pressed, Some(KeyCode::G))]
        #[test_case(WinitKeyCode::KeyH, ElementState::Pressed, Some(KeyCode::H))]
        #[test_case(WinitKeyCode::KeyI, ElementState::Pressed, Some(KeyCode::I))]
        #[test_case(WinitKeyCode::KeyJ, ElementState::Pressed, Some(KeyCode::J))]
        #[test_case(WinitKeyCode::KeyK, ElementState::Pressed, Some(KeyCode::K))]
        #[test_case(WinitKeyCode::KeyL, ElementState::Pressed, Some(KeyCode::L))]
        #[test_case(WinitKeyCode::KeyM, ElementState::Pressed, Some(KeyCode::M))]
        #[test_case(WinitKeyCode::KeyN, ElementState::Pressed, Some(KeyCode::N))]
        #[test_case(WinitKeyCode::KeyO, ElementState::Pressed, Some(KeyCode::O))]
        #[test_case(WinitKeyCode::KeyP, ElementState::Pressed, Some(KeyCode::P))]
        #[test_case(WinitKeyCode::KeyQ, ElementState::Pressed, Some(KeyCode::Q))]
        #[test_case(WinitKeyCode::KeyR, ElementState::Pressed, Some(KeyCode::R))]
        #[test_case(WinitKeyCode::KeyS, ElementState::Pressed, Some(KeyCode::S))]
        #[test_case(WinitKeyCode::KeyT, ElementState::Pressed, Some(KeyCode::T))]
        #[test_case(WinitKeyCode::KeyU, ElementState::Pressed, Some(KeyCode::U))]
        #[test_case(WinitKeyCode::KeyV, ElementState::Pressed, Some(KeyCode::V))]
        #[test_case(WinitKeyCode::KeyW, ElementState::Pressed, Some(KeyCode::W))]
        #[test_case(WinitKeyCode::KeyX, ElementState::Pressed, Some(KeyCode::X))]
        #[test_case(WinitKeyCode::KeyY, ElementState::Pressed, Some(KeyCode::Y))]
        #[test_case(WinitKeyCode::KeyZ, ElementState::Pressed, Some(KeyCode::Z))]
        #[test_case(WinitKeyCode::KeyA, ElementState::Released, Some(KeyCode::A))]
        #[test_case(WinitKeyCode::KeyB, ElementState::Released, Some(KeyCode::B))]
        #[test_case(WinitKeyCode::KeyC, ElementState::Released, Some(KeyCode::C))]
        #[test_case(WinitKeyCode::KeyD, ElementState::Released, Some(KeyCode::D))]
        #[test_case(WinitKeyCode::KeyE, ElementState::Released, Some(KeyCode::E))]
        #[test_case(WinitKeyCode::KeyF, ElementState::Released, Some(KeyCode::F))]
        #[test_case(WinitKeyCode::KeyG, ElementState::Released, Some(KeyCode::G))]
        #[test_case(WinitKeyCode::KeyH, ElementState::Released, Some(KeyCode::H))]
        #[test_case(WinitKeyCode::KeyI, ElementState::Released, Some(KeyCode::I))]
        #[test_case(WinitKeyCode::KeyJ, ElementState::Released, Some(KeyCode::J))]
        #[test_case(WinitKeyCode::KeyK, ElementState::Released, Some(KeyCode::K))]
        #[test_case(WinitKeyCode::KeyL, ElementState::Released, Some(KeyCode::L))]
        #[test_case(WinitKeyCode::KeyM, ElementState::Released, Some(KeyCode::M))]
        #[test_case(WinitKeyCode::KeyN, ElementState::Released, Some(KeyCode::N))]
        #[test_case(WinitKeyCode::KeyO, ElementState::Released, Some(KeyCode::O))]
        #[test_case(WinitKeyCode::KeyP, ElementState::Released, Some(KeyCode::P))]
        #[test_case(WinitKeyCode::KeyQ, ElementState::Released, Some(KeyCode::Q))]
        #[test_case(WinitKeyCode::KeyR, ElementState::Released, Some(KeyCode::R))]
        #[test_case(WinitKeyCode::KeyS, ElementState::Released, Some(KeyCode::S))]
        #[test_case(WinitKeyCode::KeyT, ElementState::Released, Some(KeyCode::T))]
        #[test_case(WinitKeyCode::KeyU, ElementState::Released, Some(KeyCode::U))]
        #[test_case(WinitKeyCode::KeyV, ElementState::Released, Some(KeyCode::V))]
        #[test_case(WinitKeyCode::KeyW, ElementState::Released, Some(KeyCode::W))]
        #[test_case(WinitKeyCode::KeyX, ElementState::Released, Some(KeyCode::X))]
        #[test_case(WinitKeyCode::KeyY, ElementState::Released, Some(KeyCode::Y))]
        #[test_case(WinitKeyCode::KeyZ, ElementState::Released, Some(KeyCode::Z))]
        fn should_convert_key_events(
            key_code: WinitKeyCode,
            state: ElementState,
            expected_keycode: Option<crate::KeyCode>,
        ) {
            let var_name = RawKeyEvent {
                physical_key: PhysicalKey::Code(key_code),
                state,
            };
            let event = Event::DeviceEvent::<()> {
                // SAFETY: Don't pass this to any winit functions.
                device_id: unsafe { DeviceId::dummy() },
                event: DeviceEvent::Key(var_name),
            };

            let input = winit_to_input(event).expect("Input was not converted");
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
    }
}
