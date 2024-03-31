use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Wolf Engine - Graphics Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Failed to create the window");
    let mut graphics = None;
    let _ = event_loop.run(|event, window_target| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
        Event::Resumed => {
            let window_size = window.inner_size();
            let (width, height) = (window_size.width, window_size.height);
            graphics = Some(
                pollster::block_on(
                    wolf_engine::graphics::init().build(Some((&window, (width, height)))),
                )
                .expect("Failed to create the graphics context"),
            );
        }
        _ => (),
    });
}
