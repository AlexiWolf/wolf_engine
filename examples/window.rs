use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine_events::event_loop::EventLoop;
use wolf_engine_window::{backend::WindowSystem, event::WindowEvent, WindowSettings};

fn main() {
    let window_system = wolf_engine_winit::init().unwrap();
    let context = window_system.context();

    let mut canvas = None;
    let mut window = context.create_window(
        WindowSettings::default()
            .with_title("Wolf Engine - Window Example")
            .with_size((800, 600)),
    );

    window_system.run(|event| {
        if let Some(window_event) = event.downcast_ref::<WindowEvent>() {
            match window_event {
                WindowEvent::Started => {
                    println!("Hello, world!");
                    context.create_window(
                        WindowSettings::default()
                            .with_title("Wolf Engine - Window Example")
                            .with_size((800, 600)),
                    );
                }
                WindowEvent::EventsCleared => {
                    window.redraw();
                }
                WindowEvent::WindowCreated(_, _) => {
                    canvas = Some({
                        let handle = window.handle().unwrap();
                        let (width, height) = window.size();
                        let surface_texture = SurfaceTexture::new(width, height, &handle);
                        let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
                        pixels.clear_color(Color::RED);
                        pixels
                    });
                }
                WindowEvent::WindowRedrawRequested(_) => {
                    if let Some(pixels) = &canvas {
                        pixels.render().unwrap();
                    }
                }
                WindowEvent::WindowResized(_, width, height) => {
                    if let Some(pixels) = &mut canvas {
                        pixels.resize_buffer(*width, *height).unwrap();
                        pixels.resize_surface(*width, *height).unwrap();
                    }
                }
                WindowEvent::Input(_, input) => println!("Input into window: {:?}", input),
                WindowEvent::WindowClosed(_) => context.exit(),
                WindowEvent::Exited => println!("Goodbye, World!"),
                _ => (),
            }
        }
    });
}
