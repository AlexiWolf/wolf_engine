use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::WindowEvent;

fn main() {
    let window = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .with_resizable(false)
        .build();

    let mut pixels = {
        let (width, height) = window.size();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    pixels.clear_color(Color::RED);

    window.run(|event, _window| match event {
        WindowEvent::Resume => println!("Hello, world!"),
        WindowEvent::Render => pixels.render().unwrap(),
        WindowEvent::Closed => println!("Goodbye, World!"),
        _ => (),
    });
}
