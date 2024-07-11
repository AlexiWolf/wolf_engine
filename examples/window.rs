use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::WindowEvent;

fn main() {
    let mut pixels = None;
    let context = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .with_resizable(true)
        .build()
        .unwrap();

    context.run(|event, context| match event {
        WindowEvent::Resumed => {
            println!("Hello, world!");
            pixels = Some({
                let window = context.window();
                let (width, height) = window.size();
                let surface_texture = SurfaceTexture::new(width, height, &window);
                let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
                pixels.clear_color(Color::RED);
                pixels
            });
        }
        WindowEvent::RedrawRequested => {
            if let Some(pixels) = &pixels {
                pixels.render().unwrap();
            }
        }
        WindowEvent::Resized(width, height) => {
            if let Some(pixels) = &mut pixels {
                pixels.resize_buffer(width, height).unwrap();
                pixels.resize_surface(width, height).unwrap();
            }
        }
        WindowEvent::Input(input) => println!("{:?}", input),
        WindowEvent::Closed => println!("Goodbye, World!"),
        _ => (),
    });
}
