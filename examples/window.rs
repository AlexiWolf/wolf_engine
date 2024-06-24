use pixels::{Pixels, SurfaceTexture};
use wolf_engine::window::WindowEvent;

fn main() {
    let window_context = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .with_resizable(false)
        .build();

    let mut pixels = None;

    window_context.run(|event, window| match event {
        WindowEvent::Resume => {
            pixels = Some({
                let (width, height) = window.size();
                let surface_texture = SurfaceTexture::new(width, height, &window);
                Pixels::new(width, height, surface_texture).unwrap()
            })
        }
        WindowEvent::Render => (),
        WindowEvent::Closed => (),
        _ => (),
    });
}
