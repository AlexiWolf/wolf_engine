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

    'main: loop {
        while let Some(event) = event_queue.next_event() {
            match event {
                WindowEvent::CloseRequested { .. } => break 'main,
                _ => (),
            }
        }
    }
}
