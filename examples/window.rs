use wolf_engine::prelude::*;
use wolf_engine::window::WindowSettings;
use wolf_engine_window::WindowEvent;

pub fn main() {
    let (mut event_queue, context) = wolf_engine::window::init().unwrap();
    let window = context.create_window(WindowSettings::default());

    'main: loop {
        while let Some(event) = event_queue.next_event() {
            match event {
                WindowEvent::CloseRequested { .. } => break 'main,
                _ => (),
            }
        }
    }
}
