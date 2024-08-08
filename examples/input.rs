use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowAttributes,
};
use wolf_engine::input::ToInput;
use wolf_engine_input::Input;

#[allow(deprecated)]
pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut window = None;
    let _ = event_loop.run(|event, window_target| {
        if let Some(input) = event.to_input() {
            process_input(&input);
        }
        match event {
            Event::Resumed => {
                window = Some(
                    window_target
                        .create_window(
                            WindowAttributes::default()
                                .with_title("Wolf Engine - Input Example")
                                .with_inner_size(PhysicalSize::new(800, 600))
                                .with_resizable(false),
                        )
                        .unwrap(),
                );
            }
            Event::AboutToWait => {
                window_target.listen_device_events(winit::event_loop::DeviceEvents::Always)
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            _ => (),
        }
    });
}

fn process_input(input: &Input) {
    match input {
        Input::Keyboard {
            state,
            scancode,
            keycode,
            is_repeat,
        } => println!("Key: {state:?}, {scancode:?}, {keycode:?}, {is_repeat:?}"),
        Input::MouseMove { x, y } => println!("Mouse Moved: {x}, {y}"),
        Input::RawMouseMove { delta_x, delta_y } => {
            println!("Raw Mouse Moved: {delta_x}, {delta_y}")
        }
        Input::MouseButton { state, button } => println!("Mouse Button: {button:?} {state:?}"),
        Input::MouseScroll { delta_x, delta_y } => println!("Mouse Scrolled: {delta_x} {delta_y}"),
    }
}
