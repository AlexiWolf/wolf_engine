use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use wolf_engine::input::ToInput;
use wolf_engine_input::Input;

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new()
        .with_title("Wolf Engine - Keyboard Input Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let _ = event_loop.run(|event, window_target| {
        if let Some(input) = event.to_input() {
            process_input(&input);
        }
        match event {
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
        Input::KeyDown {
            scancode,
            keycode,
            is_repeat,
        } => todo!(),
        Input::KeyUp { scancode, keycode } => todo!(),
        Input::RawKeyDown { scancode, keycode } => todo!(),
        Input::RawKeyUp { scancode, keycode } => todo!(),
    }
}
