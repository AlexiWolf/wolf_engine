#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Key {
    pub scancode: u32,
    pub keycode: Option<KeyCode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyCode {}

#[cfg(feature = "winit")]
pub mod winit {
    use crate::Input;
    use winit::event::Event;

    pub fn winit_to_input<UserEvent>(event: Event<UserEvent>) -> Option<Input> {
        None
    }

    #[cfg(test)]
    mod winit_conversion_tests {
        use ::winit::{
            event::{DeviceEvent, DeviceId, ElementState, Event, RawKeyEvent},
            keyboard::{KeyCode, PhysicalKey},
        };
        use test_case::test_case;

        use super::*;
        use crate::*;

        #[test_case(KeyCode::KeyA, ElementState::Pressed => Some(Input::KeyDown(Key { scancode: 0, keycode: None })))]
        #[test_case(KeyCode::KeyA, ElementState::Released=> Some(Input::KeyUp(Key { scancode: 0, keycode: None })))]
        fn should_convert_key_events(key_code: KeyCode, state: ElementState) -> Option<Input> {
            let var_name = RawKeyEvent {
                physical_key: PhysicalKey::Code(key_code),
                state,
            };
            let event = Event::DeviceEvent::<()> {
                // SAFETY: Don't pass this to any winit functions.
                device_id: unsafe { DeviceId::dummy() },
                event: DeviceEvent::Key(var_name),
            };

            winit_to_input(event)
        }
    }
}
