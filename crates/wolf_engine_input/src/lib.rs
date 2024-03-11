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
pub enum KeyCode {}

#[cfg(feature = "winit")]
pub mod winit {
    use crate::{Input, Key};
    use winit::{
        event::{DeviceEvent, ElementState, Event},
        platform::scancode::PhysicalKeyExtScancode,
    };

    pub fn winit_to_input<UserEvent>(event: Event<UserEvent>) -> Option<Input> {
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::Key(key_event),
                ..
            } => match key_event.state {
                ElementState::Pressed => Some(Input::KeyDown(Key {
                    scancode: key_event.physical_key.to_scancode().unwrap_or(0),
                    keycode: None,
                })),
                ElementState::Released => Some(Input::KeyUp(Key {
                    scancode: key_event.physical_key.to_scancode().unwrap_or(0),
                    keycode: None,
                })),
            },
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

        #[test_case(WinitKeyCode::KeyA, ElementState::Pressed, None)]
        #[test_case(WinitKeyCode::KeyA, ElementState::Released, None)]
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
