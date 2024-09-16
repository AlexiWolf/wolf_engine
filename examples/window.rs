use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine_events::{dynamic::AnyEvent, mpsc};
use wolf_engine_window::{
    event::{Event, WindowEvent},
    WindowSettings,
};

fn main() {
    let (event_sender, event_reciever) = mpsc::event_queue::<AnyEvent>();
    let window_context = wolf_engine::window::init(event_sender.clone());

    let mut canvas = None;
    let window = window_context.create_window(
        WindowSettings::default()
            .with_title("Wolf Engine - Window Example")
            .with_size((800, 600)),
    );

    wolf_engine_winit::run(event_reciever, |event| match event {
        Event::Started => {
            println!("Hello, world!");
        }
        Event::EventsCleared => {
            window.redraw();
        }
        Event::WindowEvent(_window_id, event) => match event {
            WindowEvent::Created(_window_result) => {
                canvas = Some({
                    let (width, height) = window.size();
                    let surface_texture = SurfaceTexture::new(width, height, window.raw_handle());
                    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
                    pixels.clear_color(Color::RED);
                    pixels
                });
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &canvas {
                    pixels.render().unwrap();
                }
            }
            WindowEvent::Resized(width, height) => {
                if let Some(pixels) = &mut canvas {
                    pixels.resize_buffer(width, height).unwrap();
                    pixels.resize_surface(width, height).unwrap();
                }
            }
            WindowEvent::Input(input) => println!("Input into window: {:?}", input),
            WindowEvent::Closed => context.exit(),
            _ => (),
        },
        Event::Exited => println!("Goodbye, World!"),
        _ => (),
    })
    .unwrap();
}
