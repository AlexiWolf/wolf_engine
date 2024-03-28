use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use wolf_engine_graphics::GraphicsContext;

struct Context {
    window: Option<Window>,
    graphics: Option<GraphicsContext>,
}

pub fn main() {
    let mut context = Context {
        window: None,
        graphics: None,
    };
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run(|event, window_target| handle_event(&mut context, event, window_target));
}

fn handle_event(context: &mut Context, event: Event<()>, event_loop: &EventLoopWindowTarget<()>) {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => event_loop.exit(),
        Event::Resumed => {
            context.window = Some(
                WindowBuilder::new()
                    .with_title("Wolf Engine - Graphics Example")
                    .with_inner_size(PhysicalSize::new(800, 600))
                    .with_resizable(false)
                    .build(event_loop)
                    .unwrap(),
            );
            context.graphics = Some(pollster::block_on(wolf_engine::graphics::init().unwrap()));
        }
        _ => (),
    }
}
