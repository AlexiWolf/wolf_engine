use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine_events::EventLoop;
use wolf_engine_window::{event::WindowEvent, WindowBackend, WindowSettings};

fn main() {
    let window_system = wolf_engine_winit::init().unwrap();
    let window_context = window_system.context();

    let mut canvas = None;
    let window = window_context.create_window(
        WindowSettings::default()
            .with_title("Wolf Engine - Window Example")
            .with_size((800, 600))
            .with_resizable(false),
    );

    window_system.run(|event| {
        if let Some(event) = event.downcast_ref::<WindowEvent>() {
            match event {
                WindowEvent::Started => {
                    println!("Hello, world!");
                }
                WindowEvent::EventsCleared => {
                    window.redraw();
                }
                WindowEvent::Input(_, input) => println!("Input into window: {:?}", input),
                WindowEvent::WindowReady(_, _window_result) => {
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
                WindowEvent::WindowResized(_, (width, height)) => {
                    if let Some(pixels) = &mut canvas {
                        pixels.resize_buffer(*width, *height).unwrap();
                        pixels.resize_surface(*width, *height).unwrap();
                    }
                }
                WindowEvent::WindowCloseRequested(_) => window_context.exit(),
                WindowEvent::Exited => println!("Goodbye, World!"),
                _ => (),
            }
        }
    });
}
