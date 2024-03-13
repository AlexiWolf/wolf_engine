use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use wolf_engine::input::ToInput;

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new()
        .with_title("Wolf Engine - Keyboard Input Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let _ = event_loop.run(|event, window_target| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
        Event::WindowEvent { .. } => {
            if let Some(input) = event.to_input() {
                println!("{:?}", input);
            }
        }
        _ => (),
    });
}
