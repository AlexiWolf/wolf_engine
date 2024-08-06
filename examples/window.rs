use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::event::WindowEvent;
use wolf_engine_window::WindowSettings;

fn main() {
    let context = wolf_engine::window::init().build().unwrap();

    let mut pixels = None;
    let mut window = None;

    context.run(|event, context| match event {
        WindowEvent::Resumed => {
            println!("Hello, world!");
            window = Some(
                context
                    .create_window(
                        WindowSettings::default()
                            .with_title("Wolf Engine - Window Example")
                            .with_size((800, 600))
                            .with_resizable(true),
                    )
                    .expect("window creation succeeded"),
            );
            pixels = Some({
                let window = window.as_ref().unwrap();
                let (width, height) = window.size();
                let surface_texture = SurfaceTexture::new(width, height, window);
                let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
                pixels.clear_color(Color::RED);
                pixels
            });
        }
        WindowEvent::RedrawRequested(_) => {
            if let Some(pixels) = &pixels {
                pixels.render().unwrap();
            }
        }
        WindowEvent::Resized(_, width, height) => {
            if let Some(pixels) = &mut pixels {
                pixels.resize_buffer(width, height).unwrap();
                pixels.resize_surface(width, height).unwrap();
            }
        }
        WindowEvent::Input(_, input) => println!("Input into window: {:?}", input),
        WindowEvent::Closed(_) => context.exit(),
        WindowEvent::Exited => println!("Goodbye, World!"),
        _ => (),
    });
}
