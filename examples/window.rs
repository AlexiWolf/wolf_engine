use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use wolf_engine::prelude::*;
use wolf_engine::window::WindowSettings;
use wolf_engine_window::WindowEvent;

pub fn main() {
    let (mut event_queue, context) = wolf_engine::window::init().unwrap();

    let mut window = context.create_window(
        WindowSettings::default()
            .with_title("Wolf Engine - Window Example")
            .with_size((1280, 720)),
    );
    let mut pixels = {
        let (width, height) = window.size().unwrap();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    pixels.clear_color(Color::RED);

    while window.is_open() {
        window.redraw();
        while let Some(event) = event_queue.next_event() {
            match event {
                WindowEvent::RedrawRequested { .. } => pixels.render().unwrap(),
                WindowEvent::CloseRequested { .. } => window.close().unwrap(),
                WindowEvent::Resized { width, height, .. } => {
                    pixels.resize_surface(width, height).unwrap()
                }
                _ => (),
            }
        }
    }
}
