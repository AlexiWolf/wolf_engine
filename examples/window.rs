use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::WindowEvent;

fn main() {
    let window = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .with_resizable(true)
        .build();

    let mut pixels = {
        let (width, height) = window.size();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    pixels.clear_color(Color::RED);

    window.run(|event, _window| match event {
        WindowEvent::Resumed => println!("Hello, world!"),
        WindowEvent::RedrawRequested => pixels.render().unwrap(),
        WindowEvent::Resized(width, height) => {
            pixels.resize_buffer(width, height).unwrap();
            pixels.resize_surface(width, height).unwrap();
        }
        WindowEvent::Closed => println!("Goodbye, World!"),
        _ => (),
    });
}
