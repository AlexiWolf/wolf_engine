use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use wolf_engine_graphics::{wgpu, GraphicsContext};
pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Wolf Engine - Graphics Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Failed to create the window");
    let mut graphics: Option<GraphicsContext> = None;
    let _ = event_loop.run(|event, window_target| match event {
        Event::NewEvents(_) => window.request_redraw(),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            if let Some(graphics) = graphics.as_mut() {
                let mut frame = graphics.new_frame().unwrap();
                graphics.clear(
                    &mut frame,
                    wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    },
                );
                graphics.present(frame);
            }
        }
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
