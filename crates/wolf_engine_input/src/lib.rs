pub enum Input {
}


#[cfg(feature = "winit")]
pub mod winit {
    #[cfg(test)]
    mod winit_conversion_tests {
        use test_case::test_case;
        #[test_case(KeyCode::KeyA, ElementState::Pressed => Some(Input::KeyDown(Key { scancode: 0, keycode: None })))]
        #[test_case(KeyCode::KeyA, ElementState::Released=> Some(Input::KeyUp(Key { scancode: 0, keycode: None })))]
        fn should_convert_key_events(key_code: KeyCode, state: ElementState) -> Option<Input> {
            None
        }
    }
}
