use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use wolf_engine::prelude::*;
use wolf_engine::window::WindowSettings;
use wolf_engine_window::WindowEvent;

pub fn main() {
    let (mut event_queue, context) = wolf_engine::window::init().unwrap();

    let window = context.create_window(WindowSettings::default());
    let mut pixels = {
        let (width, height) = window.size();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    pixels.clear_color(Color::RED);

    'main: loop {
        pixels.render().unwrap();
        while let Some(event) = event_queue.next_event() {
            match event {
                WindowEvent::CloseRequested { .. } => break 'main,
                WindowEvent::Resized { width, height, .. } => {
                    pixels.resize_surface(width, height).unwrap()
                }
                _ => (),
            }
        }
    }
}
